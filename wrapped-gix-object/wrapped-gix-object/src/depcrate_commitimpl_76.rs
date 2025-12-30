// Generated macro for impl_76 (impl)
macro_rules! Depcrate_commitimpl_76 {
() => {
// Module: crate::commit
// Provides: {"impl_76"}
// Dependencies: {}
# [doc = " Lifecycle"] impl < 'a > CommitRef < 'a > { # [doc = " Deserialize a commit from the given `data` bytes while avoiding most allocations."] pub fn from_bytes (mut data : & 'a [u8]) -> Result < CommitRef < 'a > , crate :: decode :: Error > { let input = & mut data ; match decode :: commit . parse_next (input) { Ok (tag) => Ok (tag) , Err (err) => Err (crate :: decode :: Error :: with_err (err , input)) , } } }
};
}
