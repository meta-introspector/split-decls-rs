macro_rules! deps {
    () => {
        FutureOrOutput!();
        Inner!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < Fut > Inner < Fut > where Fut : Future , { # [doc = " Safety: callers must first ensure that `self.inner.state`"] # [doc = " is `COMPLETE`"] unsafe fn output (& self) -> & Fut :: Output { match unsafe { & * self . future_or_output . get () } { FutureOrOutput :: Output (item) => item , FutureOrOutput :: Future (_) => unreachable ! () , } } }
    };
}

impl_99!()