// Generated macro for impl_86 (impl)
macro_rules! Depcrate_common_compile_cimpl_86 {
() => {
// Module: crate::common::compile_c
// Provides: {"impl_86"}
// Dependencies: {}
impl CompilationCommandBuilder { pub fn new () -> Self { Self { compiler : String :: new () , target : None , cxx_toolchain_dir : None , arch_flags : Vec :: new () , optimization : "2" . to_string () , project_root : None , extra_flags : Vec :: new () , } } pub fn set_compiler (mut self , compiler : & str) -> Self { self . compiler = compiler . to_string () ; self } pub fn set_target (mut self , target : & str) -> Self { self . target = Some (target . to_string ()) ; self } pub fn set_cxx_toolchain_dir (mut self , path : Option < & str >) -> Self { self . cxx_toolchain_dir = path . map (| p | p . to_string ()) ; self } pub fn add_arch_flags < 'a > (mut self , flags : impl IntoIterator < Item = & 'a str >) -> Self { self . arch_flags . extend (flags . into_iter () . map (| s | s . to_owned ())) ; self } pub fn set_opt_level (mut self , optimization : & str) -> Self { self . optimization = optimization . to_string () ; self } # [doc = " Sets the root path of all the generated test files."] pub fn set_project_root (mut self , path : & str) -> Self { self . project_root = Some (path . to_string ()) ; self } pub fn add_extra_flags < 'a > (mut self , flags : impl IntoIterator < Item = & 'a str >) -> Self { self . extra_flags . extend (flags . into_iter () . map (| s | s . to_owned ())) ; self } pub fn add_extra_flag (self , flag : & str) -> Self { self . add_extra_flags ([flag]) } }
};
}
