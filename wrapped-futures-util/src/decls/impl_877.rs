macro_rules! deps {
    () => {
        Pending!();
        Dequeue!();
        Task!();
        Empty!();
        FuturesUnordered!();
        Ready!();
    };
}

macro_rules! impl_877 {
    () => {
        deps!();
        impl < Fut : Future > Stream for FuturesUnordered < Fut > { type Item = Fut :: Output ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let len = self . len () ; let mut polled = 0 ; let mut yielded = 0 ; self . ready_to_run_queue . waker . register (cx . waker ()) ; loop { let task = match unsafe { self . ready_to_run_queue . dequeue () } { Dequeue :: Empty => { if self . is_empty () { * self . is_terminated . get_mut () = true ; return Poll :: Ready (None) ; } else { return Poll :: Pending ; } } Dequeue :: Inconsistent => { cx . waker () . wake_by_ref () ; return Poll :: Pending ; } Dequeue :: Data (task) => task , } ; debug_assert ! (task != self . ready_to_run_queue . stub ()) ; let future = match unsafe { & mut * (* task) . future . get () } { Some (future) => future , None => { let task = unsafe { Arc :: from_raw (task) } ; debug_assert_eq ! (task . next_all . load (Relaxed) , self . pending_next_all ()) ; unsafe { debug_assert ! ((* task . prev_all . get ()) . is_null ()) ; } continue ; } } ; let task = unsafe { self . unlink (task) } ; let prev = task . queued . swap (false , SeqCst) ; assert ! (prev) ; struct Bomb < 'a , Fut > { queue : & 'a mut FuturesUnordered < Fut > , task : Option < Arc < Task < Fut > > > , } impl < Fut > Drop for Bomb < '_ , Fut > { fn drop (& mut self) { if let Some (task) = self . task . take () { self . queue . release_task (task) ; } } } let mut bomb = Bomb { task : Some (task) , queue : & mut * self } ; let res = { let task = bomb . task . as_ref () . unwrap () ; task . woken . store (false , Relaxed) ; let waker = unsafe { Task :: waker_ref (task) } ; let mut cx = Context :: from_waker (& waker) ; let future = unsafe { Pin :: new_unchecked (future) } ; future . poll (& mut cx) } ; polled += 1 ; match res { Poll :: Pending => { let task = bomb . task . take () . unwrap () ; yielded += task . woken . load (Relaxed) as usize ; bomb . queue . link (task) ; if yielded >= 2 || polled == len { cx . waker () . wake_by_ref () ; return Poll :: Pending ; } continue ; } Poll :: Ready (output) => return Poll :: Ready (Some (output)) , } } } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } }
    };
}

impl_877!()