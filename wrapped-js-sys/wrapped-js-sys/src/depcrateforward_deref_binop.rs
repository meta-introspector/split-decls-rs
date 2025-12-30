// Generated macro for forward_deref_binop (macro)
macro_rules! Depcrateforward_deref_binop {
() => {
// Module: crate
// Provides: {"forward_deref_binop"}
// Dependencies: {}
macro_rules ! forward_deref_binop { (impl $ imp : ident , $ method : ident for $ t : ty) => { impl <'a > $ imp <$ t > for &'a $ t { type Output = <&'static $ t as $ imp <&'static $ t >>:: Output ; # [inline] fn $ method (self , other : $ t) -> Self :: Output { $ imp ::$ method (self , & other) } } impl $ imp <&$ t > for $ t { type Output = <&'static $ t as $ imp <&'static $ t >>:: Output ; # [inline] fn $ method (self , other : &$ t) -> Self :: Output { $ imp ::$ method (& self , other) } } impl $ imp <$ t > for $ t { type Output = <&'static $ t as $ imp <&'static $ t >>:: Output ; # [inline] fn $ method (self , other : $ t) -> Self :: Output { $ imp ::$ method (& self , & other) } } } ; }
};
}
