// Generated macro for from_to_typenum (macro)
macro_rules! Depcrate_base_dimensionfrom_to_typenum {
() => {
// Module: crate::base::dimension
// Provides: {"from_to_typenum"}
// Dependencies: {}
macro_rules ! from_to_typenum (($ ($ D : ident , $ VAL : expr) ;* $ (;) *) => { $ (pub type $ D = Const <$ VAL >; impl ToTypenum for Const <$ VAL > { type Typenum = typenum ::$ D ; } impl ToConst for typenum ::$ D { type Const = Const <$ VAL >; } impl IsNotStaticOne for $ D { } # [doc = " The constant dimension"] # [doc = stringify ! ($ VAL)] # [doc = " ."] pub const $ D : $ D = Const ::<$ VAL >;) * }) ;
};
}
