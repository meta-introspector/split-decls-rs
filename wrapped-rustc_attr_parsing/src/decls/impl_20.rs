macro_rules! deps {
    () => {
        EvalConfigResult!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl EvalConfigResult { pub fn as_bool (& self) -> bool { match self { EvalConfigResult :: True => true , EvalConfigResult :: False { .. } => false , } } }
    };
}

impl_20!()