// Generated macro for impl_121 (impl)
macro_rules! Depcrate_fallbackimpl_121 {
() => {
// Module: crate::fallback
// Provides: {"impl_121"}
// Dependencies: {}
impl Drop for TokenStream { fn drop (& mut self) { let mut stack = Vec :: new () ; let mut current = match self . inner . get_mut () { Some (inner) => inner . take () . into_iter () , None => return , } ; loop { while let Some (token) = current . next () { let group = match token { TokenTree :: Group (group) => group . inner , _ => continue , } ; # [cfg (wrap_proc_macro)] let group = match group { crate :: imp :: Group :: Fallback (group) => group , crate :: imp :: Group :: Compiler (_) => continue , } ; let mut group = group ; if let Some (inner) = group . stream . inner . get_mut () { stack . push (current) ; current = inner . take () . into_iter () ; } } match stack . pop () { Some (next) => current = next , None => return , } } } }
};
}
