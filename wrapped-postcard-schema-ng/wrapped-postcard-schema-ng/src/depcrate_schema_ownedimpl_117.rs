// Generated macro for impl_117 (impl)
macro_rules! Depcrate_schema_ownedimpl_117 {
() => {
// Module: crate::schema::owned
// Provides: {"impl_117"}
// Dependencies: {}
impl OwnedDataModelType { # [doc = " Convert an `[OwnedDataModelType]` to a pseudo-Rust type format"] pub fn to_pseudocode (& self) -> String { let mut buf = String :: new () ; super :: fmt :: fmt_owned_dmt_to_buf (self , & mut buf , true) ; buf } # [doc = " Collect all types used recursively by this type"] # [cfg (feature = "use-std")] pub fn all_used_types (& self) -> HashSet < Self > { let mut buf = HashSet :: new () ; super :: fmt :: discover_tys (self , & mut buf) ; buf } }
};
}
