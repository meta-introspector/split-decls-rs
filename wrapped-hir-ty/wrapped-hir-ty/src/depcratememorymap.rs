// Generated macro for MemoryMap (enum)
macro_rules! DepcrateMemoryMap {
() => {
// Module: crate
// Provides: {"MemoryMap"}
// Dependencies: {}
# [doc = " A constant can have reference to other things. Memory map job is holding"] # [doc = " the necessary bits of memory of the const eval session to keep the constant"] # [doc = " meaningful."] # [derive (Debug , Default , Clone , PartialEq , Eq)] pub enum MemoryMap < 'db > { # [default] Empty , Simple (Box < [u8] >) , Complex (Box < ComplexMemoryMap < 'db > >) , }
};
}
