// Generated macro for impl_158 (impl)
macro_rules! Depcrateimpl_158 {
() => {
// Module: crate
// Provides: {"impl_158"}
// Dependencies: {}
impl < 'a > MaybeStaticStr < 'a > { # [inline] fn get (& self) -> & 'a str { match * self { MaybeStaticStr :: Static (s) => s , MaybeStaticStr :: Borrowed (s) => s , } } }
};
}
