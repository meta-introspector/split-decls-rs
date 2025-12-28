macro_rules! deps {
    () => {
        ProcMacro!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl PartialEq for ProcMacro { fn eq (& self , other : & Self) -> bool { let Self { name , kind , expander , disabled } = self ; let Self { name : other_name , kind : other_kind , expander : other_expander , disabled : other_disabled , } = other ; name == other_name && kind == other_kind && expander == other_expander && disabled == other_disabled } }
    };
}

impl_173!();