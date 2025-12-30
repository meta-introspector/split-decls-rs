// Generated macro for impl_574 (impl)
macro_rules! Depcrate_dynamic_directiveimpl_574 {
() => {
// Module: crate::dynamic::directive
// Provides: {"impl_574"}
// Dependencies: {}
impl Directive { # [doc = " Create a directive usage"] pub fn new (name : impl Into < String >) -> Self { Self { name : name . into () , args : IndexMap :: default () , } } # [doc = " Add an argument to the directive"] # [inline] pub fn argument (mut self , name : impl Into < String > , value : Value) -> Self { self . args . insert (name . into () , value) ; self } }
};
}
