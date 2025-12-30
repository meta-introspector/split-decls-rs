// Generated macro for get_lib_name (function)
macro_rules! Depcrate_runtestget_lib_name {
() => {
// Module: crate::runtest
// Provides: {"get_lib_name"}
// Dependencies: {}
# [doc = " The platform-specific library name"] fn get_lib_name (name : & str , aux_type : AuxType) -> Option < String > { match aux_type { AuxType :: Bin => None , AuxType :: Lib => Some (format ! ("lib{name}.rlib")) , AuxType :: Dylib | AuxType :: ProcMacro => Some (dylib_name (name)) , } }
};
}
