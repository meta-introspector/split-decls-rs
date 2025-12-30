// Generated macro for test_emplace_no_drop (function)
macro_rules! Depcrate_teststest_emplace_no_drop {
() => {
// Module: crate::tests
// Provides: {"test_emplace_no_drop"}
// Dependencies: {}
# [test] fn test_emplace_no_drop () { use alloc :: { borrow :: ToOwned , string :: String } ; struct Foo < 'a > (& 'a String) ; impl Drop for Foo < '_ > { fn drop (& mut self) { panic ! ("Dropped") ; } } let mut blink = Blink :: new () ; let s = "Hello" . to_owned () ; let foo = blink . emplace_no_drop () . value (Foo (& s)) ; assert_eq ! (foo . 0 , "Hello") ; let world = blink . put ("World" . to_owned ()) ; foo . 0 = world ; blink . reset () ; }
};
}
