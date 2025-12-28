macro_rules! deps {
    () => {
        ProcMacroExpander!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl PartialEq for dyn ProcMacroExpander { fn eq (& self , other : & Self) -> bool { self . eq_dyn (other) } }
    };
}

impl_160!()