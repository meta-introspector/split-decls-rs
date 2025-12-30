// Generated macro for repr_for (function)
macro_rules! Depcrate_utilsrepr_for {
() => {
// Module: crate::utils
// Provides: {"repr_for"}
// Dependencies: {}
# [doc = " Returns the repr attribute to be applied to the resultant ULE or VarULE type"] pub fn repr_for (f : & Fields) -> TokenStream2 { if f . len () == 1 { quote ! (transparent) } else { quote ! (C , packed) } }
};
}
