macro_rules! deps {
    () => {
        FutureOrOutput!();
        Pending!();
        Shared!();
        Ready!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < Fut > Future for Shared < Fut > where Fut : Future , Fut :: Output : Clone , { type Output = Fut :: Output ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = & mut * self ; let inner = this . inner . take () . expect ("Shared future polled again after completion") ; if inner . notifier . state . load (Acquire) == COMPLETE { return unsafe { Poll :: Ready (inner . take_or_clone_output ()) } ; } inner . record_waker (& mut this . waker_key , cx) ; match inner . notifier . state . compare_exchange (IDLE , POLLING , SeqCst , SeqCst) . unwrap_or_else (| x | x) { IDLE => { } POLLING => { this . inner = Some (inner) ; return Poll :: Pending ; } COMPLETE => { return unsafe { Poll :: Ready (inner . take_or_clone_output ()) } ; } POISONED => panic ! ("inner future panicked during poll") , _ => unreachable ! () , } let waker = waker_ref (& inner . notifier) ; let mut cx = Context :: from_waker (& waker) ; struct Reset < 'a > { state : & 'a AtomicUsize , did_not_panic : bool , } impl Drop for Reset < '_ > { fn drop (& mut self) { if ! self . did_not_panic { self . state . store (POISONED , SeqCst) ; } } } let mut reset = Reset { state : & inner . notifier . state , did_not_panic : false } ; let output = { let future = unsafe { match & mut * inner . future_or_output . get () { FutureOrOutput :: Future (fut) => Pin :: new_unchecked (fut) , _ => unreachable ! () , } } ; let poll_result = future . poll (& mut cx) ; reset . did_not_panic = true ; match poll_result { Poll :: Pending => { if inner . notifier . state . compare_exchange (POLLING , IDLE , SeqCst , SeqCst) . is_ok () { drop (reset) ; this . inner = Some (inner) ; return Poll :: Pending ; } else { unreachable ! () } } Poll :: Ready (output) => output , } } ; unsafe { * inner . future_or_output . get () = FutureOrOutput :: Output (output) ; } inner . notifier . state . store (COMPLETE , SeqCst) ; # [cfg (feature = "std")] let mut wakers_guard = inner . notifier . wakers . lock () . unwrap () ; # [cfg (not (feature = "std"))] let mut wakers_guard = inner . notifier . wakers . lock () ; let mut wakers = wakers_guard . take () . unwrap () ; for waker in wakers . drain () . flatten () { waker . wake () ; } drop (reset) ; drop (wakers_guard) ; unsafe { Poll :: Ready (inner . take_or_clone_output ()) } } }
    };
}

impl_102!()