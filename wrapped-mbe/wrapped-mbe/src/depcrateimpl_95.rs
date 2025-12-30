// Generated macro for impl_95 (impl)
macro_rules! Depcrateimpl_95 {
() => {
// Module: crate
// Provides: {"impl_95"}
// Dependencies: {}
impl < T : Default , E > From < Result < T , E > > for ValueResult < T , E > { fn from (result : Result < T , E >) -> Self { result . map_or_else (Self :: only_err , Self :: ok) } }
};
}
