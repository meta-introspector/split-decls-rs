// Generated macro for impl_97 (impl)
macro_rules! Depcrate_hello_worldimpl_97 {
() => {
// Module: crate::hello_world
// Provides: {"impl_97"}
// Dependencies: {}
impl Writeable for FormattedHelloWorld < '_ > { fn write_to < W : core :: fmt :: Write + ? Sized > (& self , sink : & mut W) -> core :: fmt :: Result { self . data . message . write_to (sink) } fn writeable_borrow (& self) -> Option < & str > { self . data . message . writeable_borrow () } fn writeable_length_hint (& self) -> writeable :: LengthHint { self . data . message . writeable_length_hint () } }
};
}
