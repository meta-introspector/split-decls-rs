macro_rules! deps {
    () => {
        LocalFutureObj!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < T > Drop for LocalFutureObj < '_ , T > { fn drop (& mut self) { unsafe { (self . drop_fn) (self . future) } } }
    };
}

impl_38!()