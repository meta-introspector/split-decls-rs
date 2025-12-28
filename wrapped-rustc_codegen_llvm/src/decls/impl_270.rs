macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        impl < 'll , 'tcx > CodegenCx < 'll , 'tcx > { pub (crate) fn coverageinfo_finalize (& mut self) { mapgen :: finalize (self) } # [doc = " Returns the section name to use when embedding per-function coverage information"] # [doc = " in the object file, according to the target's object file format. LLVM's coverage"] # [doc = " tools use information from this section when producing coverage reports."] # [doc = ""] # [doc = " Typical values are:"] # [doc = " - `__llvm_covfun` on Linux"] # [doc = " - `__LLVM_COV,__llvm_covfun` on macOS (includes `__LLVM_COV,` segment prefix)"] # [doc = " - `.lcovfun$M` on Windows (includes `$M` sorting suffix)"] fn covfun_section_name (& self) -> & CStr { self . coverage_cx () . covfun_section_name . get_or_init (| | llvm_cov :: covfun_section_name (self . llmod)) } # [doc = " For LLVM codegen, returns a function-specific `Value` for a global"] # [doc = " string, to hold the function name passed to LLVM intrinsic"] # [doc = " `instrprof.increment()`. The `Value` is only created once per instance."] # [doc = " Multiple invocations with the same instance return the same `Value`."] # [doc = ""] # [doc = " This has the side-effect of causing coverage codegen to consider this"] # [doc = " function \"used\", making it eligible to emit an associated covfun record."] fn ensure_pgo_func_name_var (& self , instance : Instance < 'tcx >) -> & 'll llvm :: Value { debug ! ("getting pgo_func_name_var for instance={:?}" , instance) ; let mut pgo_func_name_var_map = self . coverage_cx () . pgo_func_name_var_map . borrow_mut () ; pgo_func_name_var_map . entry (instance) . or_insert_with (| | { let llfn = self . get_fn (instance) ; let mangled_fn_name : & str = self . tcx . symbol_name (instance) . name ; llvm_cov :: create_pgo_func_name_var (llfn , mangled_fn_name) }) } }
    };
}

impl_270!();