// Generated macro for mkserialize (macro)
macro_rules! Depcrate_value_sermkserialize {
() => {
// Module: crate::value::ser
// Provides: {"mkserialize"}
// Dependencies: {}
macro_rules ! mkserialize { ($ ($ f : ident ($ v : ty)) ,+ $ (,) ?) => { $ (# [inline] fn $ f (self , v : $ v) -> Result < Self :: Ok , Self :: Error > { Ok (v . into ()) }) + } ; }
};
}
