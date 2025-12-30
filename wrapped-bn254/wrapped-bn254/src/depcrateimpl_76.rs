// Generated macro for impl_76 (impl)
macro_rules! Depcrateimpl_76 {
() => {
// Module: crate
// Provides: {"impl_76"}
// Dependencies: {}
impl From < u64 > for AltBn128Error { fn from (v : u64) -> AltBn128Error { match v { 1 => AltBn128Error :: InvalidInputData , 2 => AltBn128Error :: GroupError , 3 => AltBn128Error :: SliceOutOfBounds , 4 => AltBn128Error :: TryIntoVecError (Vec :: new ()) , 5 => AltBn128Error :: ProjectiveToG1Failed , _ => AltBn128Error :: UnexpectedError , } } }
};
}
