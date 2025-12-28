macro_rules! deps {
    () => {
        ConstEvalError!();
    };
}

macro_rules! impl_446 {
    () => {
        deps!();
        impl < 'db > From < MirEvalError < 'db > > for ConstEvalError < 'db > { fn from (value : MirEvalError < 'db >) -> Self { ConstEvalError :: MirEvalError (value) } }
    };
}

impl_446!()