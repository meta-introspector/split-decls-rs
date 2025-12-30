// Generated macro for impl_49 (impl)
macro_rules! Depcrateimpl_49 {
() => {
// Module: crate
// Provides: {"impl_49"}
// Dependencies: {}
impl Queue { fn was_created (& self) -> bool { self . events . front () . is_some_and (| event | { matches ! (event . kind , EventKind :: Create (_) | EventKind :: Modify (ModifyKind :: Name (RenameMode :: To))) }) } fn was_removed (& self) -> bool { self . events . front () . is_some_and (| event | { matches ! (event . kind , EventKind :: Remove (_) | EventKind :: Modify (ModifyKind :: Name (RenameMode :: From))) }) } }
};
}
