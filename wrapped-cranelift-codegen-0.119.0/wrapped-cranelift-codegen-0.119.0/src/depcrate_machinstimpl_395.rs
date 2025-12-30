// Generated macro for impl_395 (impl)
macro_rules! Depcrate_machinstimpl_395 {
() => {
// Module: crate::machinst
// Provides: {"impl_395"}
// Dependencies: {}
impl CompiledCode { # [doc = " If available, return information about the code layout in the"] # [doc = " final machine code: the offsets (in bytes) of each basic-block"] # [doc = " start, and all basic-block edges."] pub fn get_code_bb_layout (& self) -> (Vec < usize > , Vec < (usize , usize) >) { (self . bb_starts . iter () . map (| & off | off as usize) . collect () , self . bb_edges . iter () . map (| & (from , to) | (from as usize , to as usize)) . collect () ,) } # [doc = " Creates unwind information for the function."] # [doc = ""] # [doc = " Returns `None` if the function has no unwind information."] # [cfg (feature = "unwind")] pub fn create_unwind_info (& self , isa : & dyn crate :: isa :: TargetIsa ,) -> CodegenResult < Option < crate :: isa :: unwind :: UnwindInfo > > { use crate :: isa :: unwind :: UnwindInfoKind ; let unwind_info_kind = match isa . triple () . operating_system { target_lexicon :: OperatingSystem :: Windows => UnwindInfoKind :: Windows , _ => UnwindInfoKind :: SystemV , } ; self . create_unwind_info_of_kind (isa , unwind_info_kind) } # [doc = " Creates unwind information for the function using the supplied"] # [doc = " \"kind\". Supports cross-OS (but not cross-arch) generation."] # [doc = ""] # [doc = " Returns `None` if the function has no unwind information."] # [cfg (feature = "unwind")] pub fn create_unwind_info_of_kind (& self , isa : & dyn crate :: isa :: TargetIsa , unwind_info_kind : crate :: isa :: unwind :: UnwindInfoKind ,) -> CodegenResult < Option < crate :: isa :: unwind :: UnwindInfo > > { isa . emit_unwind_info (self , unwind_info_kind) } }
};
}
