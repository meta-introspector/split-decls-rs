// Generated macro for CXIdxCXXClassDeclInfo (struct)
macro_rules! DepcrateCXIdxCXXClassDeclInfo {
() => {
// Module: crate
// Provides: {"CXIdxCXXClassDeclInfo"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] # [repr (C)] pub struct CXIdxCXXClassDeclInfo { pub declInfo : * const CXIdxDeclInfo , pub bases : * const * const CXIdxBaseClassInfo , pub numBases : c_uint , }
};
}
