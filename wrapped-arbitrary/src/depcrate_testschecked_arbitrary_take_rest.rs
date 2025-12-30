// Generated macro for checked_arbitrary_take_rest (function)
macro_rules! Depcrate_testschecked_arbitrary_take_rest {
() => {
// Module: crate::tests
// Provides: {"checked_arbitrary_take_rest"}
// Dependencies: {}
# [doc = " Like `checked_arbitrary()`, but calls `arbitrary_take_rest()` instead of `arbitrary()`."] fn checked_arbitrary_take_rest < 'a , T : Arbitrary < 'a > > (u : Unstructured < 'a >) -> Result < T > { let (min , _) = T :: size_hint (0) ; let len_before = u . len () ; let result = T :: arbitrary_take_rest (u) ; if result . is_ok () { assert ! (len_before >= min , "incorrect minimum size: indicated {}, worked with {}" , min , len_before) ; } result }
};
}
