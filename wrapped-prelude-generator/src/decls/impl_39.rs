macro_rules! deps {
    () => {
        FunctionDecl!();
        Declaration!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl Declaration for FunctionDecl { fn name (& self) -> & str { & self . name } }
    };
}

impl_39!();