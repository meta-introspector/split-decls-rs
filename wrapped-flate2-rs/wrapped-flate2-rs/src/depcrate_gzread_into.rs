// Generated macro for read_into (function)
macro_rules! Depcrate_gzread_into {
() => {
// Module: crate::gz
// Provides: {"read_into"}
// Dependencies: {}
fn read_into < R : Read > (r : & mut R , buffer : & mut [u8]) -> Result < usize > { debug_assert ! (! buffer . is_empty ()) ; match r . read (buffer) { Ok (0) => Err (ErrorKind :: UnexpectedEof . into ()) , Ok (n) => Ok (n) , Err (ref e) if e . kind () == ErrorKind :: Interrupted => Ok (0) , Err (e) => Err (e) , } }
};
}
