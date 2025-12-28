macro_rules! deps {
    () => {
        QueuedSpawn!();
        Access!();
        Execution!();
        State!();
        Scheduler!();
        Thread!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl Scheduler { # [doc = " Create an execution"] pub (crate) fn new (capacity : usize) -> Scheduler { Scheduler { max_threads : capacity , } } # [doc = " Access the execution"] pub (crate) fn with_execution < F , R > (f : F) -> R where F : FnOnce (& mut Execution) -> R , { Self :: with_state (| state | f (state . execution)) } # [doc = " Perform a context switch"] pub (crate) fn switch () { use std :: future :: Future ; use std :: pin :: Pin ; use std :: ptr ; use std :: task :: { Context , RawWaker , RawWakerVTable , Waker } ; unsafe fn noop_clone (_ : * const ()) -> RawWaker { unreachable ! () } unsafe fn noop (_ : * const ()) { } let mut switch = async { generator :: yield_with (()) } ; let switch = unsafe { Pin :: new_unchecked (& mut switch) } ; let raw_waker = RawWaker :: new (ptr :: null () , & RawWakerVTable :: new (noop_clone , noop , noop , noop) ,) ; let waker = unsafe { Waker :: from_raw (raw_waker) } ; let mut cx = Context :: from_waker (& waker) ; assert ! (switch . poll (& mut cx) . is_ready ()) ; } pub (crate) fn spawn (stack_size : Option < usize > , f : Box < dyn FnOnce () >) { Self :: with_state (| state | state . queued_spawn . push_back (QueuedSpawn { stack_size , f })) ; } pub (crate) fn run < F > (& mut self , execution : & mut Execution , f : F) where F : FnOnce () + Send + 'static , { let mut threads = Vec :: new () ; threads . push (spawn_thread (Box :: new (f) , None)) ; threads [0] . resume () ; loop { if execution . threads . is_complete () { for thread in & mut threads { thread . resume () ; assert ! (thread . is_done ()) ; } return ; } let active = execution . threads . active_id () ; let mut queued_spawn = Self :: tick (& mut threads [active . as_usize ()] , execution) ; while let Some (th) = queued_spawn . pop_front () { assert ! (threads . len () < self . max_threads) ; let thread_id = threads . len () ; let QueuedSpawn { f , stack_size } = th ; threads . push (spawn_thread (f , stack_size)) ; threads [thread_id] . resume () ; } } } fn tick (thread : & mut Thread , execution : & mut Execution) -> VecDeque < QueuedSpawn > { let mut queued_spawn = VecDeque :: new () ; let state = RefCell :: new (State { execution , queued_spawn : & mut queued_spawn , }) ; STATE . set (unsafe { transmute_lt (& state) } , | | { thread . resume () ; }) ; queued_spawn } fn with_state < F , R > (f : F) -> R where F : FnOnce (& mut State < '_ >) -> R , { if ! STATE . is_set () { panic ! ("cannot access Loom execution state from outside a Loom model. \
            are you accessing a Loom synchronization primitive from outside a Loom test (a call to `model` or `check`)?") } STATE . with (| state | f (& mut state . borrow_mut ())) } }
    };
}

impl_139!()