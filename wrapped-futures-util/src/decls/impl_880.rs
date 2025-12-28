macro_rules! deps {
    () => {
        FuturesUnordered!();
    };
}

macro_rules! impl_880 {
    () => {
        deps!();
        impl < Fut > Drop for FuturesUnordered < Fut > { fn drop (& mut self) { struct LeakQueueOnDrop < 'a , Fut > (& 'a mut FuturesUnordered < Fut >) ; impl < Fut > Drop for LeakQueueOnDrop < '_ , Fut > { fn drop (& mut self) { mem :: forget (Arc :: clone (& self . 0 . ready_to_run_queue)) ; } } let guard = LeakQueueOnDrop (self) ; while ! guard . 0 . head_all . get_mut () . is_null () { let head = * guard . 0 . head_all . get_mut () ; let task = unsafe { guard . 0 . unlink (head) } ; guard . 0 . release_task (task) ; } mem :: forget (guard) ; } }
    };
}

impl_880!();