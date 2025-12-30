// Generated macro for assert_generates (function)
macro_rules! Depcrate_testsassert_generates {
() => {
// Module: crate::tests
// Provides: {"assert_generates"}
// Dependencies: {}
# [doc = " Assert that the given expected values are all generated."] # [doc = ""] # [doc = " Exhaustively enumerates all buffers up to length 10 containing the"] # [doc = " following bytes: `0x00`, `0x01`, `0x61` (aka ASCII 'a'), and `0xff`"] fn assert_generates < T > (expected_values : impl IntoIterator < Item = T >) where T : Clone + Debug + Hash + Eq + for < 'a > Arbitrary < 'a > , { let expected_values : HashSet < _ > = expected_values . into_iter () . collect () ; let mut arbitrary_expected = expected_values . clone () ; let mut arbitrary_take_rest_expected = expected_values ; let bytes = [0 , 1 , b'a' , 0xff] ; let max_len = 10 ; let mut buf = Vec :: with_capacity (max_len) ; let mut g = exhaustigen :: Gen :: new () ; while ! g . done () { let len = g . gen (max_len) ; buf . clear () ; buf . extend (std :: iter :: repeat_with (| | { let index = g . gen (bytes . len () - 1) ; bytes [index] }) . take (len) ,) ; let mut u = Unstructured :: new (& buf) ; let val = T :: arbitrary (& mut u) . unwrap () ; arbitrary_expected . remove (& val) ; let u = Unstructured :: new (& buf) ; let val = T :: arbitrary_take_rest (u) . unwrap () ; arbitrary_take_rest_expected . remove (& val) ; if arbitrary_expected . is_empty () && arbitrary_take_rest_expected . is_empty () { return ; } } panic ! ("failed to generate all expected values!\n\n\
         T::arbitrary did not generate: {arbitrary_expected:#?}\n\n\
         T::arbitrary_take_rest did not generate {arbitrary_take_rest_expected:#?}") }
};
}
