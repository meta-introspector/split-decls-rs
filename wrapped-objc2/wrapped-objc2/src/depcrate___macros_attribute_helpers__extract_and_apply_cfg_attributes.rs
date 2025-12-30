// Generated macro for __extract_and_apply_cfg_attributes (macro)
macro_rules! Depcrate___macros_attribute_helpers__extract_and_apply_cfg_attributes {
() => {
// Module: crate::__macros::attribute_helpers
// Provides: {"__extract_and_apply_cfg_attributes"}
// Dependencies: {}
# [doc = " Parse the given attributes, and gate the output on any `cfg` attributes"] # [doc = " that were present in the set."] # [doc = ""] # [doc = " This is implemented as a tt-muncher, taking the following arguments:"] # [doc = " - The attributes to be processed"] # [doc = " - The output that the `cfg` attributes will be attached to"] # [doc (hidden)] # [macro_export] macro_rules ! __extract_and_apply_cfg_attributes { { () $ ($ output : tt) * } => { $ ($ output) * } ; { (# [cfg $ ($ args : tt) *] $ ($ rest : tt) *) $ ($ output : tt) * } => { # [cfg $ ($ args) *] { $ crate :: __extract_and_apply_cfg_attributes ! { ($ ($ rest) *) $ ($ output) * } } } ; { (# [$ ($ m_ignored : tt) *] $ ($ rest : tt) *) $ ($ output : tt) * } => { $ crate :: __extract_and_apply_cfg_attributes ! { ($ ($ rest) *) $ ($ output) * } } ; }
};
}
