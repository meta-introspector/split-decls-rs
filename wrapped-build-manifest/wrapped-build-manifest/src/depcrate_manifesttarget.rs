// Generated macro for Target (struct)
macro_rules! Depcrate_manifestTarget {
() => {
// Module: crate::manifest
// Provides: {"Target"}
// Dependencies: {}
# [derive (Serialize , Default)] pub (crate) struct Target { pub (crate) available : bool , pub (crate) url : Option < String > , pub (crate) hash : Option < FileHash > , pub (crate) xz_url : Option < String > , pub (crate) xz_hash : Option < FileHash > , pub (crate) components : Option < Vec < Component > > , pub (crate) extensions : Option < Vec < Component > > , }
};
}
