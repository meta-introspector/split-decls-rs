// Generated macro for impl_189 (impl)
macro_rules! Depcrate_machinst_abiimpl_189 {
() => {
// Module: crate::machinst::abi
// Provides: {"impl_189"}
// Dependencies: {}
impl ABIArgSlot { # [doc = " The type of the value that will be stored in this slot."] pub fn get_type (& self) -> ir :: Type { match self { ABIArgSlot :: Reg { ty , .. } => * ty , ABIArgSlot :: Stack { ty , .. } => * ty , } } }
};
}
