// Generated macro for discover_tys (function)
macro_rules! Depcrate_schema_fmtdiscover_tys {
() => {
// Module: crate::schema::fmt
// Provides: {"discover_tys"}
// Dependencies: {}
# [doc = " Collect unique types mentioned by this [`OwnedNamedType`]"] # [cfg (feature = "use-std")] pub fn discover_tys (ont : & OwnedNamedType , set : & mut std :: collections :: HashSet < OwnedNamedType >) { set . insert (ont . clone ()) ; discover_tys_sdm (& ont . ty , set) ; }
};
}
