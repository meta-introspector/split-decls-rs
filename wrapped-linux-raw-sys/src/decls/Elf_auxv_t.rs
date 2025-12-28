macro_rules! Elf_auxv_t {
    () => {
        # [repr (C)] # [derive (Clone)] pub struct Elf_auxv_t { pub a_type : usize , pub a_val : * mut crate :: ctypes :: c_void , }
    };
}

Elf_auxv_t!();