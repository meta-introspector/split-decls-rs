macro_rules! deps {
    () => {
        DbPanicContext!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl Drop for DbPanicContext { fn drop (& mut self) { Self :: with_ctx (| ctx | assert ! (ctx . pop () . is_some ())) ; } }
    };
}

impl_101!();