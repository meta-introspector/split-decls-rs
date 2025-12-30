// Generated macro for msrv_aliases (macro)
macro_rules! Depcrate_msrvsmsrv_aliases {
() => {
// Module: crate::msrvs
// Provides: {"msrv_aliases"}
// Dependencies: {}
macro_rules ! msrv_aliases { ($ ($ major : literal ,$ minor : literal ,$ patch : literal { $ ($ name : ident) ,* $ (,) ? }) *) => { $ ($ (pub const $ name : RustcVersion = RustcVersion { major : $ major , minor :$ minor , patch : $ patch } ;) *) * } ; }
};
}
