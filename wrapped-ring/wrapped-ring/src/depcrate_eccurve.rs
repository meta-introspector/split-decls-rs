// Generated macro for Curve (struct)
macro_rules! Depcrate_ecCurve {
() => {
// Module: crate::ec
// Provides: {"Curve"}
// Dependencies: {}
pub struct Curve { pub public_key_len : usize , pub elem_scalar_seed_len : usize , pub id : CurveID , check_private_key_bytes : fn (bytes : & [u8] , cpu : cpu :: Features) -> Result < () , error :: Unspecified > , generate_private_key : fn (rng : & dyn rand :: SecureRandom , & mut [u8] , cpu : cpu :: Features ,) -> Result < () , error :: Unspecified > , public_from_private : fn (public_out : & mut [u8] , private_key : & Seed , cpu : cpu :: Features ,) -> Result < () , error :: Unspecified > , }
};
}
