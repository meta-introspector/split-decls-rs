// Generated macro for tests (module)
macro_rules! Depcrate_tree_visittests {
() => {
// Module: crate::tree::visit
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn size_of_change () { let actual = std :: mem :: size_of :: < Change > () ; assert ! (actual <= 48 , "{actual} <= 48: this type shouldn't grow without us knowing") ; } }
};
}
