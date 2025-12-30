// Generated macro for NamedPipeBuilder (struct)
macro_rules! Depcrate_pipeNamedPipeBuilder {
() => {
// Module: crate::pipe
// Provides: {"NamedPipeBuilder"}
// Dependencies: {}
# [doc = " A builder structure for creating a new named pipe."] # [derive (Debug)] pub struct NamedPipeBuilder { name : Vec < u16 > , dwOpenMode : u32 , dwPipeMode : u32 , nMaxInstances : u32 , nOutBufferSize : u32 , nInBufferSize : u32 , nDefaultTimeOut : u32 , }
};
}
