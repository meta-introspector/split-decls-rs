// Generated macro for Seed (struct)
macro_rules! Depcrate_ed25519Seed {
() => {
// Module: crate::ed25519
// Provides: {"Seed"}
// Dependencies: {}
# [derive (Clone)] # [allow (clippy :: module_name_repetitions)] # [doc = " The seed value for the `EdDSA` signature scheme using Curve25519"] pub struct Seed < 'a > { bytes : Box < [u8] > , phantom : PhantomData < & 'a [u8] > , }
};
}
