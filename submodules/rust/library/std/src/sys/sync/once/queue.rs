mkuse!{use crate :: cell :: Cell ;}
mkuse!{use crate :: sync :: atomic :: Ordering :: { AcqRel , Acquire , Release } ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicBool , AtomicPtr } ;}
mkuse!{use crate :: sync :: poison :: once :: ExclusiveState ;}
mkuse!{use crate :: thread :: { self , Thread } ;}
mkuse!{use crate :: { fmt , ptr , sync as public } ;}
mkitem!{type StateAndQueue = * mut () ;}
mkitem!{mkstruct!{pub struct Once { state_and_queue : Atomic < * mut () > , }}}
mkitem!{mkstruct!{pub struct OnceState { poisoned : bool , set_state_on_drop_to : Cell < StateAndQueue > , }}}
mkitem!{const INCOMPLETE : usize = 0x3 ;}
mkitem!{const POISONED : usize = 0x2 ;}
mkitem!{const RUNNING : usize = 0x1 ;}
mkitem!{const COMPLETE : usize = 0x0 ;}
mkitem!{const STATE_MASK : usize = 0b11 ;}
mkitem!{const QUEUE_MASK : usize = ! STATE_MASK ;}
mkitem!{mkstruct!{# [repr (align (4))] struct Waiter { thread : Thread , signaled : Atomic < bool > , next : Cell < * const Waiter > , }}}
mkitem!{mkstruct!{struct WaiterQueue < 'a > { state_and_queue : & 'a Atomic < * mut () > , set_state_on_drop_to : StateAndQueue , }}}

macro_rules! to_queue_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_queue in module {}", module_path!());
    };
}

mkfn!{
    to_queue_introspect!();
    fn to_queue (current : StateAndQueue) -> * const Waiter { current . mask (QUEUE_MASK) . cast () }
}

macro_rules! to_state_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_state in module {}", module_path!());
    };
}

mkfn!{
    to_state_introspect!();
    fn to_state (current : StateAndQueue) -> usize { current . addr () & STATE_MASK }
}
mkitem!{mkimpl!{impl Once { # [inline] pub const fn new () -> Once { Once { state_and_queue : AtomicPtr :: new (ptr :: without_provenance_mut (INCOMPLETE)) } } # [inline] pub fn is_completed (& self) -> bool { self . state_and_queue . load (Acquire) . addr () == COMPLETE } # [inline] pub (crate) fn state (& mut self) -> ExclusiveState { match self . state_and_queue . get_mut () . addr () { INCOMPLETE => ExclusiveState :: Incomplete , POISONED => ExclusiveState :: Poisoned , COMPLETE => ExclusiveState :: Complete , _ => unreachable ! ("invalid Once state") , } } # [inline] pub (crate) fn set_state (& mut self , new_state : ExclusiveState) { * self . state_and_queue . get_mut () = match new_state { ExclusiveState :: Incomplete => ptr :: without_provenance_mut (INCOMPLETE) , ExclusiveState :: Poisoned => ptr :: without_provenance_mut (POISONED) , ExclusiveState :: Complete => ptr :: without_provenance_mut (COMPLETE) , } ; } # [cold] # [track_caller] pub fn wait (& self , ignore_poisoning : bool) { let mut current = self . state_and_queue . load (Acquire) ; loop { let state = to_state (current) ; match state { COMPLETE => return , POISONED if ! ignore_poisoning => { panic ! ("Once instance has previously been poisoned") ; } _ => { current = wait (& self . state_and_queue , current , ! ignore_poisoning) ; } } } } # [cold] # [track_caller] pub fn call (& self , ignore_poisoning : bool , init : & mut dyn FnMut (& public :: OnceState)) { let mut current = self . state_and_queue . load (Acquire) ; loop { let state = to_state (current) ; match state { COMPLETE => break , POISONED if ! ignore_poisoning => { panic ! ("Once instance has previously been poisoned") ; } POISONED | INCOMPLETE => { if let Err (new) = self . state_and_queue . compare_exchange_weak (current , current . mask (QUEUE_MASK) . wrapping_byte_add (RUNNING) , Acquire , Acquire ,) { current = new ; continue ; } let mut waiter_queue = WaiterQueue { state_and_queue : & self . state_and_queue , set_state_on_drop_to : ptr :: without_provenance_mut (POISONED) , } ; let init_state = public :: OnceState { inner : OnceState { poisoned : state == POISONED , set_state_on_drop_to : Cell :: new (ptr :: without_provenance_mut (COMPLETE)) , } , } ; init (& init_state) ; waiter_queue . set_state_on_drop_to = init_state . inner . set_state_on_drop_to . get () ; return ; } _ => { assert ! (state == RUNNING) ; current = wait (& self . state_and_queue , current , true) ; } } } } }}}

macro_rules! wait_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function wait in module {}", module_path!());
    };
}

mkfn!{
    wait_introspect!();
    fn wait (state_and_queue : & Atomic < * mut () > , mut current : StateAndQueue , return_on_poisoned : bool ,) -> StateAndQueue { let node = & Waiter { thread : thread :: current_or_unnamed () , signaled : AtomicBool :: new (false) , next : Cell :: new (ptr :: null ()) , } ; loop { let state = to_state (current) ; let queue = to_queue (current) ; if state == COMPLETE || (return_on_poisoned && state == POISONED) { return current ; } node . next . set (queue) ; if let Err (new) = state_and_queue . compare_exchange_weak (current , ptr :: from_ref (node) . wrapping_byte_add (state) as StateAndQueue , Release , Acquire ,) { current = new ; continue ; } while ! node . signaled . load (Acquire) { unsafe { node . thread . park () } } return state_and_queue . load (Acquire) ; } }
}
mkitem!{mkimpl!{# [stable (feature = "std_debug" , since = "1.16.0")] impl fmt :: Debug for Once { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Once") . finish_non_exhaustive () } }}}
mkitem!{mkimpl!{impl Drop for WaiterQueue < '_ > { fn drop (& mut self) { let current = self . state_and_queue . swap (self . set_state_on_drop_to , AcqRel) ; assert_eq ! (current . addr () & STATE_MASK , RUNNING) ; unsafe { let mut queue = to_queue (current) ; while ! queue . is_null () { let next = (* queue) . next . get () ; let thread = (* queue) . thread . clone () ; (* queue) . signaled . store (true , Release) ; thread . unpark () ; queue = next ; } } } }}}
mkitem!{mkimpl!{impl OnceState { # [inline] pub fn is_poisoned (& self) -> bool { self . poisoned } # [inline] pub fn poison (& self) { self . set_state_on_drop_to . set (ptr :: without_provenance_mut (POISONED)) ; } }}}