// Generated macro for mkvisit (macro)
macro_rules! Depcrate_value_demkvisit {
() => {
// Module: crate::value::de
// Provides: {"mkvisit"}
// Dependencies: {}
macro_rules ! mkvisit { ($ ($ f : ident ($ v : ty)) ,+ $ (,) ?) => { $ (# [inline] fn $ f < E : de :: Error > (self , v : $ v) -> Result < Self :: Value , E > { Ok (v . into ()) }) + } ; }
};
}
