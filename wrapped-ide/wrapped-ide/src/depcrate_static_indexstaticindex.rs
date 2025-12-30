// Generated macro for StaticIndex (struct)
macro_rules! Depcrate_static_indexStaticIndex {
() => {
// Module: crate::static_index
// Provides: {"StaticIndex"}
// Dependencies: {}
# [doc = " A static representation of fully analyzed source code."] # [doc = ""] # [doc = " The intended use-case is powering read-only code browsers and emitting LSIF/SCIP."] # [derive (Debug)] pub struct StaticIndex < 'a > { pub files : Vec < StaticIndexedFile > , pub tokens : TokenStore , analysis : & 'a Analysis , db : & 'a RootDatabase , def_map : FxHashMap < Definition , TokenId > , }
};
}
