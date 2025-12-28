macro_rules! deps {
    () => {
        Param!();
        PreciseCapturingArg!();
        Lifetime!();
    };
}

macro_rules! impl_275 {
    () => {
        deps!();
        impl PreciseCapturingArg < '_ > { pub fn hir_id (self) -> HirId { match self { PreciseCapturingArg :: Lifetime (lt) => lt . hir_id , PreciseCapturingArg :: Param (param) => param . hir_id , } } pub fn name (self) -> Symbol { match self { PreciseCapturingArg :: Lifetime (lt) => lt . ident . name , PreciseCapturingArg :: Param (param) => param . ident . name , } } }
    };
}

impl_275!();