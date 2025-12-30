// Generated macro for impl_2968 (impl)
macro_rules! Depcrate_egraphimpl_2968 {
() => {
// Module: crate::egraph
// Provides: {"impl_2968"}
// Dependencies: {}
impl < 'a > CtxHash < (Type , InstructionData) > for GVNContext < 'a > { fn ctx_hash < H : Hasher > (& self , state : & mut H , (ty , inst) : & (Type , InstructionData)) { std :: hash :: Hash :: hash (& ty , state) ; inst . hash (state , self . value_lists) ; } }
};
}
