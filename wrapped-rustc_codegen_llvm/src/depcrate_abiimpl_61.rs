// Generated macro for impl_61 (impl)
macro_rules! Depcrate_abiimpl_61 {
() => {
// Module: crate::abi
// Provides: {"impl_61"}
// Dependencies: {}
impl AbiBuilderMethods for Builder < '_ , '_ , '_ > { fn get_param (& mut self , index : usize) -> Self :: Value { llvm :: get_param (self . llfn () , index as c_uint) } }
};
}
