// Generated macro for impl_68 (impl)
macro_rules! Depcrate_linuximpl_68 {
() => {
// Module: crate::linux
// Provides: {"impl_68"}
// Dependencies: {}
impl < 'a > fmt :: Debug for DebugPhdr < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let phdr = self . 0 ; f . debug_struct ("Phdr") . field ("p_type" , & phdr . p_type) . field ("p_flags" , & phdr . p_flags) . field ("p_offset" , & phdr . p_offset) . field ("p_vaddr" , & phdr . p_vaddr) . field ("p_paddr" , & phdr . p_paddr) . field ("p_filesz" , & phdr . p_filesz) . field ("p_memsz" , & phdr . p_memsz) . field ("p_align" , & phdr . p_align) . finish () } }
};
}
