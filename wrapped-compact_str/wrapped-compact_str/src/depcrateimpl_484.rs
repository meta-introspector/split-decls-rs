// Generated macro for impl_484 (impl)
macro_rules! Depcrateimpl_484 {
() => {
// Module: crate
// Provides: {"impl_484"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < CompactString > for Box < dyn std :: error :: Error > { fn from (value : CompactString) -> Self { let err1 : Box < dyn std :: error :: Error + Send + Sync > = From :: from (value) ; let err2 : Box < dyn std :: error :: Error > = err1 ; err2 } }
};
}
