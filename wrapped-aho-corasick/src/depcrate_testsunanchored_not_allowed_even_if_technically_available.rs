// Generated macro for unanchored_not_allowed_even_if_technically_available (function)
macro_rules! Depcrate_testsunanchored_not_allowed_even_if_technically_available {
() => {
// Module: crate::tests
// Provides: {"unanchored_not_allowed_even_if_technically_available"}
// Dependencies: {}
# [test] fn unanchored_not_allowed_even_if_technically_available () { let ac = AhoCorasick :: builder () . kind (Some (AhoCorasickKind :: NoncontiguousNFA)) . start_kind (StartKind :: Anchored) . build (& ["foo"]) . unwrap () ; assert ! (ac . try_find (Input :: new ("foo") . anchored (Anchored :: No)) . is_err ()) ; let ac = AhoCorasick :: builder () . kind (Some (AhoCorasickKind :: ContiguousNFA)) . start_kind (StartKind :: Anchored) . build (& ["foo"]) . unwrap () ; assert ! (ac . try_find (Input :: new ("foo") . anchored (Anchored :: No)) . is_err ()) ; let ac = AhoCorasick :: builder () . kind (Some (AhoCorasickKind :: DFA)) . start_kind (StartKind :: Anchored) . build (& ["foo"]) . unwrap () ; assert ! (ac . try_find (Input :: new ("foo") . anchored (Anchored :: No)) . is_err ()) ; }
};
}
