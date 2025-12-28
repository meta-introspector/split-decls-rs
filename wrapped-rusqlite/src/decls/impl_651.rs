macro_rules! deps {
    () => {
        VTabLogCursor!();
        VTabLog!();
    };
}

macro_rules! impl_651 {
    () => {
        deps!();
        impl VTabLogCursor < '_ > { fn vtab (& self) -> & VTabLog { unsafe { & * (self . base . pVtab as * const VTabLog) } } }
    };
}

impl_651!()