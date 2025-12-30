// Generated macro for impl_849 (impl)
macro_rules! Depcrate_core_builder_cargoimpl_849 {
() => {
// Module: crate::core::builder::cargo
// Provides: {"impl_849"}
// Dependencies: {}
impl From < Cargo > for BootstrapCommand { fn from (mut cargo : Cargo) -> BootstrapCommand { if cargo . release_build { cargo . args . insert (0 , "--release" . into ()) ; } cargo . command . args (cargo . args) ; let rustflags = & cargo . rustflags . 0 ; if ! rustflags . is_empty () { cargo . command . env ("RUSTFLAGS" , rustflags) ; } let rustdocflags = & cargo . rustdocflags . 0 ; if ! rustdocflags . is_empty () { cargo . command . env ("RUSTDOCFLAGS" , rustdocflags) ; } let encoded_hostflags = cargo . hostflags . encode () ; if ! encoded_hostflags . is_empty () { cargo . command . env ("RUSTC_HOST_FLAGS" , encoded_hostflags) ; } if ! cargo . allow_features . is_empty () { cargo . command . env ("RUSTC_ALLOW_FEATURES" , cargo . allow_features) ; } cargo . command } }
};
}
