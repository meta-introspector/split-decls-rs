// Generated macro for VcpuEvents (struct)
macro_rules! Depcrate_x86_64VcpuEvents {
() => {
// Module: crate::x86_64
// Provides: {"VcpuEvents"}
// Dependencies: {}
# [repr (C)] # [derive (Copy)] pub struct VcpuEvents { pub exception : StructUnnamed5 , pub interrupt : StructUnnamed6 , pub nmi : StructUnnamed7 , pub sipi_vector : u32 , pub flags : u32 , pub reserved : [u32 ; 10usize] , }
};
}
