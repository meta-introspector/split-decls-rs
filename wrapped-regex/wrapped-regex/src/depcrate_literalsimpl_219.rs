// Generated macro for impl_219 (impl)
macro_rules! Depcrate_literalsimpl_219 {
() => {
// Module: crate::literals
// Provides: {"impl_219"}
// Dependencies: {}
impl Matcher { fn prefixes (lits : & syntax :: Literals) -> Self { let sset = SingleByteSet :: prefixes (& lits) ; Matcher :: new (lits , sset) } fn suffixes (lits : & syntax :: Literals) -> Self { let sset = SingleByteSet :: suffixes (& lits) ; Matcher :: new (lits , sset) } fn new (lits : & syntax :: Literals , sset : SingleByteSet) -> Self { if lits . literals () . is_empty () { return Matcher :: Empty ; } if sset . dense . len () >= 26 { return Matcher :: Empty ; } if sset . complete { return Matcher :: Bytes (sset) ; } if lits . literals () . len () == 1 { let lit = lits . literals () [0] . to_vec () ; return Matcher :: Single (SingleSearch :: new (lit)) ; } let is_aho_corasick_fast = sset . dense . len () == 1 && sset . all_ascii ; if is_teddy_128_available () && ! is_aho_corasick_fast { const MAX_TEDDY_LITERALS : usize = 32 ; if lits . literals () . len () <= MAX_TEDDY_LITERALS { if let Some (ted) = Teddy :: new (lits) { return Matcher :: Teddy128 (ted) ; } } } let pats = lits . literals () . to_owned () ; Matcher :: AC (AcAutomaton :: new (pats) . into_full ()) } }
};
}
