// Generated macro for impl_77 (impl)
macro_rules! Depcrate_scopeimpl_77 {
() => {
// Module: crate::scope
// Provides: {"impl_77"}
// Dependencies: {}
impl < A , T > Scope < '_ , 'static , A , T > { # [doc = " yield and get the send para"] # [inline] pub fn yield_ (& mut self , v : T) -> Option < A > { unsafe { self . yield_unsafe (v) } } # [doc = " `yield_from`"] # [doc = " the from generator must has the same type as itself"] pub fn yield_from (& mut self , g : Generator < A , T >) -> Option < A > { unsafe { self . yield_from_unsafe (g) } } }
};
}
