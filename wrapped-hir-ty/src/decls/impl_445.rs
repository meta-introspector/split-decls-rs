macro_rules! deps {
    () => {
        ConstEvalError!();
    };
}

macro_rules! impl_445 {
    () => {
        deps!();
        impl < 'db > From < MirLowerError < 'db > > for ConstEvalError < 'db > { fn from (value : MirLowerError < 'db >) -> Self { match value { MirLowerError :: ConstEvalError (_ , e) => * e , _ => ConstEvalError :: MirLowerError (value) , } } }
    };
}

impl_445!();