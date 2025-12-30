// Generated macro for impl_38 (impl)
macro_rules! Depcrateimpl_38 {
() => {
// Module: crate
// Provides: {"impl_38"}
// Dependencies: {}
impl C { fn print_intrinsics (& mut self) { self . src . c_fns ("\n// Canonical ABI intrinsics") ; self . src . c_fns ("\n") ; self . src . c_fns (r#"
                __attribute__((__weak__, __export_name__("cabi_realloc")))
                void *cabi_realloc(void *ptr, size_t old_size, size_t align, size_t new_size) {
                    (void) old_size;
                    if (new_size == 0) return (void*) align;
                    void *ret = realloc(ptr, new_size);
                    if (!ret) abort();
                    return ret;
                }
            "# ,) ; } }
};
}
