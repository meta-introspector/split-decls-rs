// Generated macro for impl_262 (impl)
macro_rules! Depcrate_debuginfo_emitimpl_262 {
() => {
// Module: crate::debuginfo::emit
// Provides: {"impl_262"}
// Dependencies: {}
impl WriterRelocate { pub (super) fn new (endian : RunTimeEndian) -> Self { WriterRelocate { relocs : Vec :: new () , writer : EndianVec :: new (endian) } } # [doc = " Perform the collected relocations to be usable for JIT usage."] # [cfg (all (feature = "jit" , not (windows)))] pub (super) fn relocate_for_jit (mut self , jit_module : & cranelift_jit :: JITModule) -> Vec < u8 > { use cranelift_module :: Module ; for reloc in self . relocs . drain (..) { match reloc . name { super :: DebugRelocName :: Section (_) => unreachable ! () , super :: DebugRelocName :: Symbol (sym) => { let addr = if sym & 1 << 31 == 0 { let func_id = FuncId :: from_u32 (sym . try_into () . unwrap ()) ; if jit_module . declarations () . get_function_decl (func_id) . name . as_deref () == Some ("rust_eh_personality") { extern "C" { fn rust_eh_personality () -> ! ; } rust_eh_personality as * const u8 } else { jit_module . get_finalized_function (func_id) } } else { jit_module . get_finalized_data (DataId :: from_u32 (u32 :: try_from (sym) . unwrap () & ! (1 << 31) ,)) . 0 } ; let val = (addr as u64 as i64 + reloc . addend) as u64 ; self . writer . write_udata_at (reloc . offset as usize , val , reloc . size) . unwrap () ; } } } self . writer . into_vec () } }
};
}
