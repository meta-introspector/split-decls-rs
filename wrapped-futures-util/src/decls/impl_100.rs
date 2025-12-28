macro_rules! deps {
    () => {
        Inner!();
        FutureOrOutput!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < Fut > Inner < Fut > where Fut : Future , Fut :: Output : Clone , { # [doc = " Registers the current task to receive a wakeup when we are awoken."] fn record_waker (& self , waker_key : & mut usize , cx : & mut Context < '_ >) { # [cfg (feature = "std")] let mut wakers_guard = self . notifier . wakers . lock () . unwrap () ; # [cfg (not (feature = "std"))] let mut wakers_guard = self . notifier . wakers . lock () ; let wakers_mut = wakers_guard . as_mut () ; let wakers = match wakers_mut { Some (wakers) => wakers , None => return , } ; let new_waker = cx . waker () ; if * waker_key == NULL_WAKER_KEY { * waker_key = wakers . insert (Some (new_waker . clone ())) ; } else { match wakers [* waker_key] { Some (ref old_waker) if new_waker . will_wake (old_waker) => { } ref mut slot => * slot = Some (new_waker . clone ()) , } } debug_assert ! (* waker_key != NULL_WAKER_KEY) ; } # [doc = " Safety: callers must first ensure that `inner.state`"] # [doc = " is `COMPLETE`"] unsafe fn take_or_clone_output (self : Arc < Self >) -> Fut :: Output { match Arc :: try_unwrap (self) { Ok (inner) => match inner . future_or_output . into_inner () { FutureOrOutput :: Output (item) => item , FutureOrOutput :: Future (_) => unreachable ! () , } , Err (inner) => unsafe { inner . output () . clone () } , } } }
    };
}

impl_100!()