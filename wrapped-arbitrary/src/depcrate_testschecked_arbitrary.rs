// Generated macro for checked_arbitrary (function)
macro_rules! Depcrate_testschecked_arbitrary {
() => {
// Module: crate::tests
// Provides: {"checked_arbitrary"}
// Dependencies: {}
# [doc = " Generates an arbitrary `T`, and checks that the result is consistent with the"] # [doc = " `size_hint()` reported by `T`."] fn checked_arbitrary < 'a , T : Arbitrary < 'a > > (u : & mut Unstructured < 'a >) -> Result < T > { let (min , max) = T :: size_hint (0) ; let len_before = u . len () ; let result = T :: arbitrary (u) ; let consumed = len_before - u . len () ; if let Some (max) = max { assert ! (consumed <= max , "incorrect maximum size: indicated {}, actually consumed {}" , max , consumed) ; } if result . is_ok () { assert ! (consumed >= min , "incorrect minimum size: indicated {}, actually consumed {}" , min , consumed) ; } result }
};
}
