// Generated macro for ComplexMemoryMap (struct)
macro_rules! DepcrateComplexMemoryMap {
() => {
// Module: crate
// Provides: {"ComplexMemoryMap"}
// Dependencies: {}
# [derive (Debug , Default , Clone , PartialEq , Eq)] pub struct ComplexMemoryMap < 'db > { memory : IndexMap < usize , Box < [u8] > , FxBuildHasher > , vtable : VTableMap < 'db > , }
};
}
