// Generated macro for generate_option (function)
macro_rules! Depcrategenerate_option {
() => {
// Module: crate
// Provides: {"generate_option"}
// Dependencies: {}
fn generate_option < T : Generate , R : rand :: Rng > (rng : & mut R) -> Option < T > { if u64 :: generate (rng) % 2 == 0 { None } else { Some (T :: generate (rng)) } }
};
}
