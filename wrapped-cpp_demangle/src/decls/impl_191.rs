macro_rules! deps {
    () => {
        BareFunctionType!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl BareFunctionType { fn ret (& self) -> & TypeHandle { & self . 0 [0] } fn args (& self) -> & FunctionArgListAndReturnType { FunctionArgListAndReturnType :: new (& self . 0) } }
    };
}

impl_191!();