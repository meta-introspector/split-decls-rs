// Generated macro for get_maybe_pointer_size (function)
macro_rules! Depcrate_builderget_maybe_pointer_size {
() => {
// Module: crate::builder
// Provides: {"get_maybe_pointer_size"}
// Dependencies: {}
# [cfg (not (feature = "master"))] fn get_maybe_pointer_size (value : RValue < '_ >) -> u32 { let type_ = value . get_type () ; if type_ . get_pointee () . is_some () { size_of :: < * const () > () as _ } else { type_ . get_size () } }
};
}
