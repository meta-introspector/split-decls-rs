// Generated macro for Field (trait)
macro_rules! Depcrate_module_lattice_algebraField {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"Field"}
// Dependencies: {}
pub trait Field : Copy + Default + Debug + PartialEq { type Int : PrimInt + Default + Debug + From < u8 > + Into < u128 > + Into < Self :: Long > + Truncate < u128 > ; type Long : PrimInt + From < Self :: Int > ; type LongLong : PrimInt ; const Q : Self :: Int ; const QL : Self :: Long ; const QLL : Self :: LongLong ; const BARRETT_SHIFT : usize ; const BARRETT_MULTIPLIER : Self :: LongLong ; fn small_reduce (x : Self :: Int) -> Self :: Int ; fn barrett_reduce (x : Self :: Long) -> Self :: Int ; }
};
}
