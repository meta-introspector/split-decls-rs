macro_rules! deps {
    () => {
        ThreadData!();
        UnparkHandleT!();
    };
}

macro_rules! ThreadParkerT {
    () => {
        deps!();
        # [doc = " Trait for the platform thread parker implementation."] # [doc = ""] # [doc = " All unsafe methods are unsafe because the Unix thread parker is based on"] # [doc = " pthread mutexes and condvars. Those primitives must not be moved and used"] # [doc = " from any other memory address than the one they were located at when they"] # [doc = " were initialized. As such, it's UB to call any unsafe method on"] # [doc = " `ThreadParkerT` if the implementing instance has moved since the last"] # [doc = " call to any of the unsafe methods."] pub trait ThreadParkerT { type UnparkHandle : UnparkHandleT ; const IS_CHEAP_TO_CONSTRUCT : bool ; fn new () -> Self ; # [doc = " Prepares the parker. This should be called before adding it to the queue."] unsafe fn prepare_park (& self) ; # [doc = " Checks if the park timed out. This should be called while holding the"] # [doc = " queue lock after `park_until` has returned false."] unsafe fn timed_out (& self) -> bool ; # [doc = " Parks the thread until it is unparked. This should be called after it has"] # [doc = " been added to the queue, after unlocking the queue."] unsafe fn park (& self) ; # [doc = " Parks the thread until it is unparked or the timeout is reached. This"] # [doc = " should be called after it has been added to the queue, after unlocking"] # [doc = " the queue. Returns true if we were unparked and false if we timed out."] unsafe fn park_until (& self , timeout : Instant) -> bool ; # [doc = " Locks the parker to prevent the target thread from exiting. This is"] # [doc = " necessary to ensure that thread-local `ThreadData` objects remain valid."] # [doc = " This should be called while holding the queue lock."] unsafe fn unpark_lock (& self) -> Self :: UnparkHandle ; }
    };
}

ThreadParkerT!();