// Generated macro for impl_161 (impl)
macro_rules! Depcrate_astimpl_161 {
() => {
// Module: crate::ast
// Provides: {"impl_161"}
// Dependencies: {}
impl < 'a , S > Definition < 'a , S > { # [doc = " Sets or resets the provided `description` for this [`Definition`]."] pub (crate) fn set_description (& mut self , description : Option < Spanning < Cow < 'a , str > > >) { match self { Self :: Operation (op) => op . item . description = description , Self :: Fragment (frag) => frag . item . description = description , } } }
};
}
