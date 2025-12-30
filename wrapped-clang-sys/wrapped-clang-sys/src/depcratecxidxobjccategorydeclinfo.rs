// Generated macro for CXIdxObjCCategoryDeclInfo (struct)
macro_rules! DepcrateCXIdxObjCCategoryDeclInfo {
() => {
// Module: crate
// Provides: {"CXIdxObjCCategoryDeclInfo"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] # [repr (C)] pub struct CXIdxObjCCategoryDeclInfo { pub containerInfo : * const CXIdxObjCContainerDeclInfo , pub objcClass : * const CXIdxEntityInfo , pub classCursor : CXCursor , pub classLoc : CXIdxLoc , pub protocols : * const CXIdxObjCProtocolRefListInfo , }
};
}
