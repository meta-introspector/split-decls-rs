macro_rules! WeakExec {
    () => {
        # [derive (Clone)] pub (crate) struct WeakExec (Weak < hyper_executor >) ;
    };
}

WeakExec!()