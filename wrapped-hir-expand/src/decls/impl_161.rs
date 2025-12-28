macro_rules! deps {
    () => {
        ProcMacroExpander!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl Eq for dyn ProcMacroExpander { }
    };
}

impl_161!()