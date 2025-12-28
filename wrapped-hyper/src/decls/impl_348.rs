macro_rules! deps {
    () => {
        Executor!();
        WeakExec!();
        AsTaskType!();
    };
}

macro_rules! impl_348 {
    () => {
        deps!();
        impl < F > crate :: rt :: Executor < F > for WeakExec where F : Future + Send + 'static , F :: Output : Send + Sync + AsTaskType , { fn execute (& self , fut : F) { if let Some (exec) = self . 0 . upgrade () { exec . spawn (hyper_task :: boxed (fut)) ; } } }
    };
}

impl_348!();