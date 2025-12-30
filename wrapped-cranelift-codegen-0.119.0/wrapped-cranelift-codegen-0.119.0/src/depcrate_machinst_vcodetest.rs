// Generated macro for test (module)
macro_rules! Depcrate_machinst_vcodetest {
() => {
// Module: crate::machinst::vcode
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use std :: mem :: size_of ; # [test] fn size_of_constant_structs () { assert_eq ! (size_of ::< Constant > () , 4) ; assert_eq ! (size_of ::< VCodeConstant > () , 4) ; assert_eq ! (size_of ::< ConstantData > () , 3 * size_of ::< usize > ()) ; assert_eq ! (size_of ::< VCodeConstantData > () , 4 * size_of ::< usize > ()) ; assert_eq ! (size_of ::< PrimaryMap < VCodeConstant , VCodeConstantData >> () , 3 * size_of ::< usize > ()) ; } }
};
}
