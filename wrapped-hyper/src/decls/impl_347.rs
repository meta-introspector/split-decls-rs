macro_rules! deps {
    () => {
        WeakExec!();
    };
}

macro_rules! impl_347 {
    () => {
        deps!();
        impl WeakExec { pub (crate) fn new () -> Self { WeakExec (Weak :: new ()) } }
    };
}

impl_347!()