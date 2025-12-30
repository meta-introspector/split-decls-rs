// Generated macro for macro_34 (macro)
macro_rules! Depcrate_combinators_collectmacro_34 {
() => {
// Module: crate::combinators::collect
// Provides: {"macro_34"}
// Dependencies: {}
pin_project ! { # [doc = " Future that resolves into a [`Collected`]."] # [doc = ""] # [doc = " [`Collected`]: crate::Collected"] pub struct Collect < T > where T : Body , T : ? Sized , { pub (crate) collected : Option < crate :: Collected < T :: Data >>, # [pin] pub (crate) body : T , } }
};
}
