// Generated macro for arrays (macro)
macro_rules! Depcrate_impls_arraysarrays {
() => {
// Module: crate::impls::arrays
// Provides: {"arrays"}
// Dependencies: {}
macro_rules ! arrays { ($ ($ len : literal $ fmt : literal ,) +) => { impl < T , const N : usize > Format for [T ; N] where T : Format { default_format ! () ; # [inline] fn _format_tag () -> Str { match N { $ ($ len => internp ! ($ fmt) ,) + _ => internp ! ("{=[?]}") , } } # [inline] fn _format_data (& self) { match N { $ ($ len) |+ => export :: fmt_array (self) , _ => export :: fmt_slice (self) , } } } } ; }
};
}
