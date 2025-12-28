macro_rules! deps {
    () => {
        VTabLog!();
    };
}

macro_rules! impl_646 {
    () => {
        deps!();
        impl Drop for VTabLog { fn drop (& mut self) { println ! ("VTabLog::drop({})" , self . i_inst) ; } }
    };
}

impl_646!();