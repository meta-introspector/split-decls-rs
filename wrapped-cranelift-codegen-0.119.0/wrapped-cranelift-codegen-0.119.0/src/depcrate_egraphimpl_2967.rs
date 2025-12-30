// Generated macro for impl_2967 (impl)
macro_rules! Depcrate_egraphimpl_2967 {
() => {
// Module: crate::egraph
// Provides: {"impl_2967"}
// Dependencies: {}
impl < 'a > CtxEq < (Type , InstructionData) , (Type , InstructionData) > for GVNContext < 'a > { fn ctx_eq (& self , (a_ty , a_inst) : & (Type , InstructionData) , (b_ty , b_inst) : & (Type , InstructionData) ,) -> bool { a_ty == b_ty && a_inst . eq (b_inst , self . value_lists) } }
};
}
