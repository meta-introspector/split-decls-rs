// Generated macro for get_dllimport (function)
macro_rules! Depcrate_commonget_dllimport {
() => {
// Module: crate::common
// Provides: {"get_dllimport"}
// Dependencies: {}
pub (crate) fn get_dllimport < 'tcx > (tcx : TyCtxt < 'tcx > , id : DefId , name : & str ,) -> Option < & 'tcx DllImport > { tcx . native_library (id) . and_then (| lib | lib . dll_imports . iter () . find (| di | di . name . as_str () == name)) }
};
}
