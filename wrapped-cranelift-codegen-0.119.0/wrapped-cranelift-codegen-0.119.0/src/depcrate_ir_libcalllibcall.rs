// Generated macro for LibCall (enum)
macro_rules! Depcrate_ir_libcallLibCall {
() => {
// Module: crate::ir::libcall
// Provides: {"LibCall"}
// Dependencies: {}
# [doc = " The name of a runtime library routine."] # [doc = ""] # [doc = " Runtime library calls are generated for Cranelift IR instructions that don't have an equivalent"] # [doc = " ISA instruction or an easy macro expansion. A `LibCall` is used as a well-known name to refer to"] # [doc = " the runtime library routine. This way, Cranelift doesn't have to know about the naming"] # [doc = " convention in the embedding VM's runtime library."] # [doc = ""] # [doc = " This list is likely to grow over time."] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub enum LibCall { # [doc = " probe for stack overflow. These are emitted for functions which need"] # [doc = " when the `enable_probestack` setting is true."] Probestack , # [doc = " ceil.f32"] CeilF32 , # [doc = " ceil.f64"] CeilF64 , # [doc = " floor.f32"] FloorF32 , # [doc = " floor.f64"] FloorF64 , # [doc = " trunc.f32"] TruncF32 , # [doc = " frunc.f64"] TruncF64 , # [doc = " nearest.f32"] NearestF32 , # [doc = " nearest.f64"] NearestF64 , # [doc = " fma.f32"] FmaF32 , # [doc = " fma.f64"] FmaF64 , # [doc = " libc.memcpy"] Memcpy , # [doc = " libc.memset"] Memset , # [doc = " libc.memmove"] Memmove , # [doc = " libc.memcmp"] Memcmp , # [doc = " Elf __tls_get_addr"] ElfTlsGetAddr , # [doc = " Elf __tls_get_offset"] ElfTlsGetOffset , # [doc = " The `pshufb` on x86 when SSSE3 isn't available."] X86Pshufb , }
};
}
