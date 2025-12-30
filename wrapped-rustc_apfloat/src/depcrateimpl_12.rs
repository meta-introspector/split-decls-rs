// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl < T : core :: fmt :: Debug > StatusAnd < T > { # [doc = " Extract the inner value if there were no errors. If there were errors, panic."] pub fn unwrap (self) -> T { assert_eq ! (self . status , Status :: OK , "called `StatusAnd::unwrap()` on an error value. Value: {:?}" , self . value) ; self . value } }
};
}
