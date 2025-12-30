// Generated macro for CXIdxEntityInfo (struct)
macro_rules! DepcrateCXIdxEntityInfo {
() => {
// Module: crate
// Provides: {"CXIdxEntityInfo"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] # [repr (C)] pub struct CXIdxEntityInfo { pub kind : CXIdxEntityKind , pub templateKind : CXIdxEntityCXXTemplateKind , pub lang : CXIdxEntityLanguage , pub name : * const c_char , pub USR : * const c_char , pub cursor : CXCursor , pub attributes : * const * const CXIdxAttrInfo , pub numAttributes : c_uint , }
};
}
