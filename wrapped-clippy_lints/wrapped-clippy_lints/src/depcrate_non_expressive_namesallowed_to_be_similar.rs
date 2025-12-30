// Generated macro for allowed_to_be_similar (function)
macro_rules! Depcrate_non_expressive_namesallowed_to_be_similar {
() => {
// Module: crate::non_expressive_names
// Provides: {"allowed_to_be_similar"}
// Dependencies: {}
# [must_use] fn allowed_to_be_similar (interned_name : & str , list : & [& str]) -> bool { list . iter () . any (| & name | interned_name . starts_with (name) || interned_name . ends_with (name)) }
};
}
