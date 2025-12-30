// Generated macro for __impl_external_bitflags (macro)
macro_rules! Depcrate_external__impl_external_bitflags {
() => {
// Module: crate::external
// Provides: {"__impl_external_bitflags"}
// Dependencies: {}
# [doc = " Implements traits from external libraries for the internal bitflags type."] # [macro_export] # [doc (hidden)] macro_rules ! __impl_external_bitflags { ($ InternalBitFlags : ident : $ T : ty , $ PublicBitFlags : ident { $ ($ (# [$ inner : ident $ ($ args : tt) *]) * const $ Flag : tt ;) * }) => { $ crate :: __impl_external_bitflags_serde ! { $ InternalBitFlags : $ T , $ PublicBitFlags { $ ($ (# [$ inner $ ($ args) *]) * const $ Flag ;) * } } $ crate :: __impl_external_bitflags_arbitrary ! { $ InternalBitFlags : $ T , $ PublicBitFlags { $ ($ (# [$ inner $ ($ args) *]) * const $ Flag ;) * } } $ crate :: __impl_external_bitflags_bytemuck ! { $ InternalBitFlags : $ T , $ PublicBitFlags { $ ($ (# [$ inner $ ($ args) *]) * const $ Flag ;) * } } } ; }
};
}
