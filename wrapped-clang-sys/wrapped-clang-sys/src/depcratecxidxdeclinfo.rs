// Generated macro for CXIdxDeclInfo (struct)
macro_rules! DepcrateCXIdxDeclInfo {
() => {
// Module: crate
// Provides: {"CXIdxDeclInfo"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] # [repr (C)] pub struct CXIdxDeclInfo { pub entityInfo : * const CXIdxEntityInfo , pub cursor : CXCursor , pub loc : CXIdxLoc , pub semanticContainer : * const CXIdxContainerInfo , pub lexicalContainer : * const CXIdxContainerInfo , pub isRedeclaration : c_int , pub isDefinition : c_int , pub isContainer : c_int , pub declAsContainer : * const CXIdxContainerInfo , pub isImplicit : c_int , pub attributes : * const * const CXIdxAttrInfo , pub numAttributes : c_uint , pub flags : c_uint , }
};
}
