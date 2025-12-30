// Generated macro for impl_25 (impl)
macro_rules! Depcrate_bit_setimpl_25 {
() => {
// Module: crate::bit_set
// Provides: {"impl_25"}
// Dependencies: {}
impl < T : Idx > BitRelations < DenseBitSet < T > > for DenseBitSet < T > { fn union (& mut self , other : & DenseBitSet < T >) -> bool { assert_eq ! (self . domain_size , other . domain_size) ; bitwise (& mut self . words , & other . words , | a , b | a | b) } fn subtract (& mut self , other : & DenseBitSet < T >) -> bool { assert_eq ! (self . domain_size , other . domain_size) ; bitwise (& mut self . words , & other . words , | a , b | a & ! b) } fn intersect (& mut self , other : & DenseBitSet < T >) -> bool { assert_eq ! (self . domain_size , other . domain_size) ; bitwise (& mut self . words , & other . words , | a , b | a & b) } }
};
}
