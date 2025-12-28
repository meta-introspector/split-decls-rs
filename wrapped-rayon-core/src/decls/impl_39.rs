macro_rules! deps {
    () => {
        Job!();
        AbortIfPanic!();
        Latch!();
        JobResult!();
        StackJob!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < L , F , R > Job for StackJob < L , F , R > where L : Latch + Sync , F : FnOnce (bool) -> R + Send , R : Send , { unsafe fn execute (this : * const ()) { unsafe { let this = & * (this as * const Self) ; let abort = unwind :: AbortIfPanic ; let func = (* this . func . get ()) . take () . unwrap () ; (* this . result . get ()) = JobResult :: call (func) ; Latch :: set (& this . latch) ; mem :: forget (abort) ; } } }
    };
}

impl_39!();