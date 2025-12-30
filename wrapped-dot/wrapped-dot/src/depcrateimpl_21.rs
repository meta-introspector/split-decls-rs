// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl Arrow { # [doc = " Return `true` if this is a default arrow."] fn is_default (& self) -> bool { self . arrows . is_empty () } # [doc = " Arrow constructor which returns an empty arrow"] pub fn none () -> Arrow { Arrow { arrows : vec ! [NoArrow] , } } # [doc = " Arrow constructor which returns a regular triangle arrow, without modifiers"] pub fn normal () -> Arrow { Arrow { arrows : vec ! [ArrowShape :: normal ()] , } } # [doc = " Arrow constructor which returns an arrow created by a given ArrowShape."] pub fn from_arrow (arrow : ArrowShape) -> Arrow { Arrow { arrows : vec ! [arrow] , } } # [doc = " Function which converts given arrow into a renderable form."] pub fn to_dot_string (& self) -> String { let mut cow = String :: new () ; for arrow in & self . arrows { cow . push_str (& arrow . to_dot_string ()) ; } cow } }
};
}
