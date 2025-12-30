// Generated macro for impl_147 (impl)
macro_rules! Depcrateimpl_147 {
() => {
// Module: crate
// Provides: {"impl_147"}
// Dependencies: {}
impl PosixDst < Abbreviation > { fn quote (& self) -> proc_macro2 :: TokenStream { let PosixDst { ref abbrev , ref offset , ref rule } = * self ; let abbrev = abbrev . as_str () ; let offset = offset . quote () ; let rule = rule . quote () ; quote ! { jiff :: shared :: PosixDst { abbrev : # abbrev , offset : # offset , rule : # rule , } } } }
};
}
