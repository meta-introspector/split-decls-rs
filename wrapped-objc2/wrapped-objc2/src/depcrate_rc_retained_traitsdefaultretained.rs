// Generated macro for DefaultRetained (trait)
macro_rules! Depcrate_rc_retained_traitsDefaultRetained {
() => {
// Module: crate::rc::retained_traits
// Provides: {"DefaultRetained"}
// Dependencies: {}
# [doc = " Helper trait to implement [`Default`] on [`Retained`]."] # [doc (alias = "DefaultId")] pub trait DefaultRetained { # [doc = " The default [`Retained`] for a type."] # [doc = ""] # [doc = " On most objects the implementation would be sending a message to the"] # [doc = " `new` selector."] fn default_retained () -> Retained < Self > ; }
};
}
