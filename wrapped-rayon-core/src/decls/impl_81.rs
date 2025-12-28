macro_rules! deps {
    () => {
        SpinLatch!();
        CoreLatch!();
        Latch!();
        Registry!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl Latch for SpinLatch < '_ > { # [inline] unsafe fn set (this : * const Self) { unsafe { let registry : & Registry = if (* this) . cross { & Arc :: clone ((* this) . registry) } else { (* this) . registry } ; let target_worker_index = (* this) . target_worker_index ; if CoreLatch :: set (& (* this) . core_latch) { registry . notify_worker_latch_is_set (target_worker_index) ; } } } }
    };
}

impl_81!()