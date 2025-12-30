// Generated macro for lookup_by_name (function)
macro_rules! Depcrate_isalookup_by_name {
() => {
// Module: crate::isa
// Provides: {"lookup_by_name"}
// Dependencies: {}
# [doc = " Look for a supported ISA with the given `name`."] # [doc = " Return a builder that can create a corresponding `TargetIsa`."] pub fn lookup_by_name (name : & str) -> Result < Builder , LookupError > { lookup (triple ! (name)) }
};
}
