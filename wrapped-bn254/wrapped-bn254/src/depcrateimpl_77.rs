// Generated macro for impl_77 (impl)
macro_rules! Depcrateimpl_77 {
() => {
// Module: crate
// Provides: {"impl_77"}
// Dependencies: {}
impl From < AltBn128Error > for u64 { fn from (v : AltBn128Error) -> u64 { match v { AltBn128Error :: InvalidInputData => 1 , AltBn128Error :: GroupError => 2 , AltBn128Error :: SliceOutOfBounds => 3 , AltBn128Error :: TryIntoVecError (_) => 4 , AltBn128Error :: ProjectiveToG1Failed => 5 , AltBn128Error :: UnexpectedError => 6 , } } }
};
}
