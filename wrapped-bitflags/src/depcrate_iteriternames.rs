// Generated macro for IterNames (struct)
macro_rules! Depcrate_iterIterNames {
() => {
// Module: crate::iter
// Provides: {"IterNames"}
// Dependencies: {}
# [doc = "\nAn iterator over flags values.\n\nThis iterator only yields flags values for contained, defined, named flags. Any remaining bits\nwon't be yielded, but can be found with the [`IterNames::remaining`] method.\n"] pub struct IterNames < B : 'static > { flags : & 'static [Flag < B >] , idx : usize , source : B , remaining : B , }
};
}
