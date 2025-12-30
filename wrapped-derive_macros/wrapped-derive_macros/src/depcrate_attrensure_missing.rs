// Generated macro for ensure_missing (function)
macro_rules! Depcrate_attrensure_missing {
() => {
// Module: crate::attr
// Provides: {"ensure_missing"}
// Dependencies: {}
pub (crate) fn ensure_missing < T : SvalAttribute > (ctxt : & str , request : T , attrs : & [Attribute]) { let key = request . key () . to_owned () ; if get_unchecked :: < T > (ctxt , request , attrs) . is_some () { panic ! ("unsupported attribute `{}` on {}" , key , ctxt) ; } }
};
}
