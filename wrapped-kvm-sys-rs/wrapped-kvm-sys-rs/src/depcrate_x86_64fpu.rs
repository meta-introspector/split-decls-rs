// Generated macro for Fpu (struct)
macro_rules! Depcrate_x86_64Fpu {
() => {
// Module: crate::x86_64
// Provides: {"Fpu"}
// Dependencies: {}
# [repr (C)] # [derive (Copy)] pub struct Fpu { pub fpr : [[u8 ; 16usize] ; 8usize] , pub fcw : u16 , pub fsw : u16 , pub ftwx : u8 , pub pad1 : u8 , pub last_opcode : u16 , pub last_ip : u64 , pub last_dp : u64 , pub xmm : [[u8 ; 16usize] ; 16usize] , pub mxcsr : u32 , pub pad2 : u32 , }
};
}
