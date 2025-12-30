// Generated macro for impl_1098 (impl)
macro_rules! Depcrate_transliterate_transliteratorimpl_1098 {
() => {
// Module: crate::transliterate::transliterator
// Provides: {"impl_1098"}
// Dependencies: {}
impl MatchData { fn new () -> Self { Self { segments : Vec :: new () , } } fn update_segment (& mut self , i : usize , s : String) { if i >= self . segments . len () { self . segments . resize_with (i + 1 , Default :: default) ; } self . segments [i] = s ; } fn get_segment (& self , i : usize) -> & str { if let Some (s) = self . segments . get (i) { return s ; } "" } }
};
}
