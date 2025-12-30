// Generated macro for get_pointer (function)
macro_rules! Depcrate_interact_sessionget_pointer {
() => {
// Module: crate::interact::session
// Provides: {"get_pointer"}
// Dependencies: {}
fn get_pointer < T > (ptr : & Option < Box < T > >) -> usize where T : ? Sized , { ptr . as_ref () . map_or (0 , | f | std :: ptr :: addr_of ! (f) as usize) }
};
}
