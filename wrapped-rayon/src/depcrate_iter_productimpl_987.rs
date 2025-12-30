// Generated macro for impl_987 (impl)
macro_rules! Depcrate_iter_productimpl_987 {
() => {
// Module: crate::iter::product
// Provides: {"impl_987"}
// Dependencies: {}
impl < P , T > Folder < T > for ProductFolder < P > where P : Product < T > + Product , { type Result = P ; fn consume (self , item : T) -> Self { ProductFolder { product : mul (self . product , iter :: once (item) . product ()) , } } fn consume_iter < I > (self , iter : I) -> Self where I : IntoIterator < Item = T > , { ProductFolder { product : mul (self . product , iter . into_iter () . product ()) , } } fn complete (self) -> P { self . product } fn full (& self) -> bool { false } }
};
}
