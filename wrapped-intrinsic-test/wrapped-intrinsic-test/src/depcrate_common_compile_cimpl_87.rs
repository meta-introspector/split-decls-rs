// Generated macro for impl_87 (impl)
macro_rules! Depcrate_common_compile_cimpl_87 {
() => {
// Module: crate::common::compile_c
// Provides: {"impl_87"}
// Dependencies: {}
impl CompilationCommandBuilder { pub fn into_cpp_compilation (self) -> CppCompilation { let mut cpp_compiler = std :: process :: Command :: new (self . compiler) ; if let Some (project_root) = self . project_root { cpp_compiler . current_dir (project_root) ; } let flags = std :: env :: var ("CPPFLAGS") . unwrap_or ("" . into ()) ; cpp_compiler . args (flags . split_whitespace ()) ; cpp_compiler . arg (format ! ("-march={}" , self . arch_flags . join ("+"))) ; cpp_compiler . arg (format ! ("-O{}" , self . optimization)) ; cpp_compiler . args (self . extra_flags) ; if let Some (target) = & self . target { cpp_compiler . arg (format ! ("--target={target}")) ; } CppCompilation (cpp_compiler) } }
};
}
