// Generated macro for size_and_align_of (function)
macro_rules! Depcrate_debuginfo_metadatasize_and_align_of {
() => {
// Module: crate::debuginfo::metadata
// Provides: {"size_and_align_of"}
// Dependencies: {}
# [doc = " Extract size and alignment from a TyAndLayout."] # [inline] fn size_and_align_of (ty_and_layout : TyAndLayout < '_ >) -> (Size , Align) { (ty_and_layout . size , ty_and_layout . align . abi) }
};
}
