// Generated macro for CXIdxEntityRefInfo (struct)
macro_rules! DepcrateCXIdxEntityRefInfo {
() => {
// Module: crate
// Provides: {"CXIdxEntityRefInfo"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] # [repr (C)] pub struct CXIdxEntityRefInfo { pub kind : CXIdxEntityRefKind , pub cursor : CXCursor , pub loc : CXIdxLoc , pub referencedEntity : * const CXIdxEntityInfo , pub parentEntity : * const CXIdxEntityInfo , pub container : * const CXIdxContainerInfo , # [doc = " Only available on `libclang` 7.0 and later."] # [cfg (feature = "clang_7_0")] pub role : CXSymbolRole , }
};
}
