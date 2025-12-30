// Generated macro for impl_623 (impl)
macro_rules! Depcrate_typesimpl_623 {
() => {
// Module: crate::types
// Provides: {"impl_623"}
// Dependencies: {}
impl MTLResourceID { # [doc = " Construct a `MTLResourceID` from an ID previously gotten via `to_raw`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The documentation for `MTLResourceID` says:"] # [doc = ""] # [doc = " > A MTLResourceID represents a specific GPU resource, mutating this"] # [doc = " > handle is undefined unless the mutation results in the value"] # [doc = " > equalling an already existing handle of the same resource type."] # [doc = ""] # [doc = " So we've tentatively marked this method as `unsafe`, with the safety"] # [doc = " requirement that the ID must be valid, i.e. have previously come from"] # [doc = " [`to_raw`][Self::to_raw] or similar."] # [doc = ""] # [doc = " If you disagree with this assessment, feel free to open an issue!"] pub const unsafe fn from_raw (id : u64) -> Self { Self { _impl : id } } # [doc = " Get the underlying data of the ID."] # [doc = ""] # [doc = " May be useful for FFI purposes."] pub const fn to_raw (self) -> u64 { self . _impl } }
};
}
