// Generated macro for intern_const_scalar (function)
macro_rules! Depcrate_constevalintern_const_scalar {
() => {
// Module: crate::consteval
// Provides: {"intern_const_scalar"}
// Dependencies: {}
# [doc = " Interns a constant scalar with the given type"] pub fn intern_const_scalar (value : ConstScalar , ty : Ty) -> Const { ConstData { ty , value : ConstValue :: Concrete (chalk_ir :: ConcreteConst { interned : value }) } . intern (Interner) }
};
}
