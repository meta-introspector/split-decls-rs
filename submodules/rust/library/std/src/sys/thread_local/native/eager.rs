mkuse!{use crate :: cell :: { Cell , UnsafeCell } ;}
mkuse!{use crate :: ptr :: { self , drop_in_place } ;}
mkuse!{use crate :: sys :: thread_local :: { abort_on_dtor_unwind , destructors } ;}
mkitem!{mkenum!{# [derive (Clone , Copy)] enum State { Initial , Alive , Destroyed , }}}
mkitem!{mkstruct!{# [allow (missing_debug_implementations)] pub struct Storage < T > { state : Cell < State > , val : UnsafeCell < T > , }}}
mkitem!{mkimpl!{impl < T > Storage < T > { pub const fn new (val : T) -> Storage < T > { Storage { state : Cell :: new (State :: Initial) , val : UnsafeCell :: new (val) } } # [doc = " Gets a pointer to the TLS value. If the TLS variable has been destroyed,"] # [doc = " a null pointer is returned."] # [doc = ""] # [doc = " The resulting pointer may not be used after thread destruction has"] # [doc = " occurred."] # [doc = ""] # [doc = " # Safety"] # [doc = " The `self` reference must remain valid until the TLS destructor is run."] # [inline] pub unsafe fn get (& self) -> * const T { match self . state . get () { State :: Alive => self . val . get () , State :: Destroyed => ptr :: null () , State :: Initial => unsafe { self . initialize () } , } } # [cold] unsafe fn initialize (& self) -> * const T { unsafe { destructors :: register (ptr :: from_ref (self) . cast_mut () . cast () , destroy :: < T >) ; } self . state . set (State :: Alive) ; self . val . get () } }}}

macro_rules! destroy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function destroy in module {}", module_path!());
    };
}

mkfn!{
    destroy_introspect!();
    # [doc = " Transition an `Alive` TLS variable into the `Destroyed` state, dropping its"] # [doc = " value."] # [doc = ""] # [doc = " # Safety"] # [doc = " * Must only be called at thread destruction."] # [doc = " * `ptr` must point to an instance of `Storage` with `Alive` state and be"] # [doc = "   valid for accessing that instance."] unsafe extern "C" fn destroy < T > (ptr : * mut u8) { abort_on_dtor_unwind (| | { let storage = unsafe { & * (ptr as * const Storage < T >) } ; storage . state . set (State :: Destroyed) ; unsafe { drop_in_place (storage . val . get ()) ; } }) }
}