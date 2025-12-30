// Generated macro for impl_24 (impl)
macro_rules! Depcrate_readobjimpl_24 {
() => {
// Module: crate::readobj
// Provides: {"impl_24"}
// Dependencies: {}
impl PrintOptions { # [doc = " Returns a new `PrintOptions` with all selectors enabled and default modifiers."] pub fn all () -> Self { Self { file : true , segments : true , sections : true , symbols : true , relocations : true , elf_dynamic : true , elf_dynamic_symbols : true , elf_notes : true , elf_versions : true , elf_attributes : true , macho_load_commands : true , macho_function_starts : true , macho_exports_trie : true , pe_rich : true , pe_base_relocs : true , pe_imports : true , pe_exports : true , pe_resources : true , string_indices : true , } } # [doc = " Returns a new `PrintOptions` with all selectors disabled and default modifiers."] pub fn none () -> Self { Self { file : false , segments : false , sections : false , symbols : false , relocations : false , elf_dynamic : false , elf_dynamic_symbols : false , elf_notes : false , elf_versions : false , elf_attributes : false , macho_load_commands : false , macho_function_starts : false , macho_exports_trie : false , pe_rich : false , pe_base_relocs : false , pe_imports : false , pe_exports : false , pe_resources : false , string_indices : true , } } }
};
}
