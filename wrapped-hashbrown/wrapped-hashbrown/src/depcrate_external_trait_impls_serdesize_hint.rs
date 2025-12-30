// Generated macro for size_hint (module)
macro_rules! Depcrate_external_trait_impls_serdesize_hint {
() => {
// Module: crate::external_trait_impls::serde
// Provides: {"size_hint"}
// Dependencies: {}
mod size_hint { use core :: cmp ; # [doc = " This presumably exists to prevent denial of service attacks."] # [doc = ""] # [doc = " Original discussion: https://github.com/serde-rs/serde/issues/1114."] # [cfg_attr (feature = "inline-more" , inline)] pub (super) fn cautious (hint : Option < usize >) -> usize { cmp :: min (hint . unwrap_or (0) , 4096) } }
};
}
