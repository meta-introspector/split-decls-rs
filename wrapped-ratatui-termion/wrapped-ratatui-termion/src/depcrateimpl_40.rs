// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
impl FromTermion < tcolor :: Bg < tcolor :: AnsiValue > > for Style { fn from_termion (value : tcolor :: Bg < tcolor :: AnsiValue >) -> Self { Self :: default () . bg (Color :: Indexed (value . 0 . 0)) } }
};
}
