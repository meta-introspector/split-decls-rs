// Generated macro for generate_hash_with_hash_fn (function)
macro_rules! Depcrategenerate_hash_with_hash_fn {
() => {
// Module: crate
// Provides: {"generate_hash_with_hash_fn"}
// Dependencies: {}
pub fn generate_hash_with_hash_fn < T , F > (entries : & [T] , hash_fn : F) -> HashState where F : Fn (& T , & HashKey) -> Hashes , { let mut generator = Generator :: new (entries . len ()) ; let mut rng = Rng :: with_seed (FIXED_SEED) ; iter :: repeat_with (| | rng . u64 (..)) . find (| key | { let hashes = entries . iter () . map (| entry | hash_fn (entry , key)) ; generator . reset (hashes) ; generator . try_generate_hash () }) . map (| key | HashState { key , disps : generator . disps , map : generator . map . into_iter () . map (| i | i . unwrap ()) . collect () , }) . expect ("failed to solve PHF") }
};
}
