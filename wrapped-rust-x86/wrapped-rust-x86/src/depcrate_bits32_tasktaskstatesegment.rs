// Generated macro for TaskStateSegment (struct)
macro_rules! Depcrate_bits32_taskTaskStateSegment {
() => {
// Module: crate::bits32::task
// Provides: {"TaskStateSegment"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] # [repr (C , packed)] pub struct TaskStateSegment { pub link : u16 , reserved0 : u16 , pub esp0 : u32 , pub ss0 : u16 , reserved1 : u16 , pub esp1 : u32 , pub ss1 : u16 , reserved2 : u16 , pub esp2 : u32 , pub ss2 : u16 , reserved3 : u16 , pub cr3 : u32 , pub eip : u32 , pub eflags : u32 , pub eax : u32 , pub ecx : u32 , pub edx : u32 , pub ebx : u32 , pub esp : u32 , pub ebp : u32 , pub esi : u32 , pub edi : u32 , pub es : u16 , reserved4 : u16 , pub cs : u16 , reserved5 : u16 , pub ss : u16 , reserved6 : u16 , pub ds : u16 , reserved7 : u16 , pub fs : u16 , reserved8 : u16 , pub gs : u16 , reserved9 : u16 , pub ldtr : u16 , reserved10 : u32 , pub iobp_offset : u16 , }
};
}
