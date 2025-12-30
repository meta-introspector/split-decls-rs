// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl FromTermion < tcolor :: Fg < tcolor :: AnsiValue > > for Style { fn from_termion (value : tcolor :: Fg < tcolor :: AnsiValue >) -> Self { Self :: default () . fg (Color :: Indexed (value . 0 . 0)) } }
};
}
