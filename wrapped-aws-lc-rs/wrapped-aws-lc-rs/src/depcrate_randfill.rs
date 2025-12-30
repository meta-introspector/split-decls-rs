// Generated macro for fill (function)
macro_rules! Depcrate_randfill {
() => {
// Module: crate::rand
// Provides: {"fill"}
// Dependencies: {}
# [doc = " Fills `dest` with random bytes."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified` if unable to fill `dest`."] pub fn fill (dest : & mut [u8]) -> Result < () , Unspecified > { if 1 != indicator_check ! (unsafe { RAND_bytes (dest . as_mut_ptr () , dest . len ()) }) { return Err (Unspecified) ; } Ok (()) }
};
}
