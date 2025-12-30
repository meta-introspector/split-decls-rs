// Generated macro for impl_constructor (macro)
macro_rules! Depcrate_series_surfaceimpl_constructor {
() => {
// Module: crate::series::surface
// Provides: {"impl_constructor"}
// Dependencies: {}
macro_rules ! impl_constructor { ($ dir : ty , $ name : ident) => { impl <'a , X , Y , Z , SurfaceFunc > SurfaceSeries <'a , X , Y , Z , $ dir , SurfaceFunc > where SurfaceFunc : Fn (<$ dir as Direction < X , Y , Z >>:: Input1Type , <$ dir as Direction < X , Y , Z >>:: Input2Type ,) -> <$ dir as Direction < X , Y , Z >>:: OutputType , { # [doc = " Implements the constructor. See [`SurfaceSeries`] for more information and examples."] pub fn $ name < IterA , IterB > (a : IterA , b : IterB , f : SurfaceFunc) -> Self where IterA : Iterator < Item = <$ dir as Direction < X , Y , Z >>:: Input1Type >, IterB : Iterator < Item = <$ dir as Direction < X , Y , Z >>:: Input2Type >, { Self :: new (a , b , f) } } } ; }
};
}
