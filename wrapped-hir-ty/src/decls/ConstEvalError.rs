macro_rules! ConstEvalError {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq)] pub enum ConstEvalError < 'db > { MirLowerError (MirLowerError < 'db >) , MirEvalError (MirEvalError < 'db >) , }
    };
}

ConstEvalError!();