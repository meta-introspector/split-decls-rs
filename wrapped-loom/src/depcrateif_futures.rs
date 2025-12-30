// Generated macro for if_futures (macro)
macro_rules! Depcrateif_futures {
() => {
// Module: crate
// Provides: {"if_futures"}
// Dependencies: {}
macro_rules ! if_futures { ($ ($ t : tt) *) => { cfg_if :: cfg_if ! { if # [cfg (feature = "futures")] { # [cfg_attr (docsrs , doc (cfg (feature = "futures")))] $ ($ t) * } } } }
};
}
