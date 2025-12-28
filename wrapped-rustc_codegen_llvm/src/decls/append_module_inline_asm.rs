macro_rules! append_module_inline_asm {
    () => {
        # [doc = " Safe wrapper for `LLVMAppendModuleInlineAsm`, which delegates to"] # [doc = " `Module::appendModuleInlineAsm`."] pub (crate) fn append_module_inline_asm < 'll > (llmod : & 'll Module , asm : & [u8]) { unsafe { LLVMAppendModuleInlineAsm (llmod , asm . as_ptr () , asm . len ()) ; } }
    };
}

append_module_inline_asm!()