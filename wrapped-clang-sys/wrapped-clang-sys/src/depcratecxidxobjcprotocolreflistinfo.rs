// Generated macro for CXIdxObjCProtocolRefListInfo (struct)
macro_rules! DepcrateCXIdxObjCProtocolRefListInfo {
() => {
// Module: crate
// Provides: {"CXIdxObjCProtocolRefListInfo"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] # [repr (C)] pub struct CXIdxObjCProtocolRefListInfo { pub protocols : * const * const CXIdxObjCProtocolRefInfo , pub numProtocols : c_uint , }
};
}
