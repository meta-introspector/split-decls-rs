macro_rules! deps {
    () => {
        ThreadData!();
    };
}

macro_rules! deadlock {
    () => {
        deps!();
        # [doc = " \\[Experimental\\] Deadlock detection"] # [doc = ""] # [doc = " Enabled via the `deadlock_detection` feature flag."] pub mod deadlock { # [cfg (feature = "deadlock_detection")] use super :: deadlock_impl ; # [cfg (feature = "deadlock_detection")] pub (super) use super :: deadlock_impl :: DeadlockData ; # [doc = " Acquire a resource identified by key in the deadlock detector"] # [doc = " Noop if `deadlock_detection` feature isn't enabled."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Call after the resource is acquired"] # [inline] pub unsafe fn acquire_resource (_key : usize) { # [cfg (feature = "deadlock_detection")] deadlock_impl :: acquire_resource (_key) ; } # [doc = " Release a resource identified by key in the deadlock detector."] # [doc = " Noop if `deadlock_detection` feature isn't enabled."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resource was already released or wasn't acquired in this thread."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Call before the resource is released"] # [inline] pub unsafe fn release_resource (_key : usize) { # [cfg (feature = "deadlock_detection")] deadlock_impl :: release_resource (_key) ; } # [doc = " Returns all deadlocks detected *since* the last call."] # [doc = " Each cycle consist of a vector of `DeadlockedThread`."] # [cfg (feature = "deadlock_detection")] # [inline] pub fn check_deadlock () -> Vec < Vec < deadlock_impl :: DeadlockedThread > > { deadlock_impl :: check_deadlock () } # [inline] pub (super) unsafe fn on_unpark (_td : & super :: ThreadData) { # [cfg (feature = "deadlock_detection")] deadlock_impl :: on_unpark (_td) ; } }
    };
}

deadlock!();