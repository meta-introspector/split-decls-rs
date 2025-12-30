// Generated macro for spanned (macro)
macro_rules! Depcrate_util_spanned_valuespanned {
() => {
// Module: crate::util::spanned_value
// Provides: {"spanned"}
// Dependencies: {}
macro_rules ! spanned { ($ trayt : ident , $ method : ident , $ syn : path) => { impl < T : $ trayt > $ trayt for SpannedValue < T > { fn $ method (value : &$ syn) -> Result < Self > { Ok (SpannedValue :: new ($ trayt ::$ method (value) . map_err (| e | e . with_span (value)) ?, value . span () ,)) } } } ; }
};
}
