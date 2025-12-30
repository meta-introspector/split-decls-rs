// Generated macro for filter_attrs (function)
macro_rules! Depcratefilter_attrs {
() => {
// Module: crate
// Provides: {"filter_attrs"}
// Dependencies: {}
fn filter_attrs (attrs : Vec < Attribute >) -> (Vec < Attribute > , Vec < SalsaAttr >) { let mut other = vec ! [] ; let mut salsa = vec ! [] ; for attr in attrs { match SalsaAttr :: try_from (attr) { Ok (it) => salsa . push (it) , Err (it) => other . push (it) , } } (other , salsa) }
};
}
