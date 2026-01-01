/* FP:profiling.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_profiling_USE_0001
/* FP:profiling.rs-0002 */ use std :: ffi :: { CStr , c_void } ;
/* FP:profiling.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_profiling_USE_0002
/* FP:profiling.rs-0004 */ use std :: os :: raw :: c_char ;
/* FP:profiling.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_profiling_USE_0003
/* FP:profiling.rs-0006 */ use std :: sync :: Arc ;
/* FP:profiling.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_profiling_USE_0004
/* FP:profiling.rs-0008 */ use measureme :: event_id :: SEPARATOR_BYTE ;
/* FP:profiling.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_profiling_USE_0005
/* FP:profiling.rs-0010 */ use measureme :: { EventId , StringComponent , StringId } ;
/* FP:profiling.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_profiling_USE_0006
/* FP:profiling.rs-0012 */ use crate :: rustc_data_structures :: profiling :: { SelfProfiler , TimingGuard } ;
/* FP:profiling.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_profiling_FN_0007
/* FP:profiling.rs-0014 */ fn llvm_args_to_string_id (profiler : & SelfProfiler , pass_name : & str , ir_name : & str) -> EventId { let pass_name = profiler . get_or_alloc_cached_string (pass_name) ; let mut components = vec ! [StringComponent :: Ref (pass_name)] ; let parentheses : & [_] = & ['(' , ')'] ; let trimmed = ir_name . trim_matches (parentheses) ; for part in trimmed . split (", ") { let demangled_ir_name = rustc_demangle :: demangle (part) . to_string () ; let ir_name = profiler . get_or_alloc_cached_string (demangled_ir_name) ; components . push (StringComponent :: Value (SEPARATOR_BYTE)) ; components . push (StringComponent :: Ref (ir_name)) ; } EventId :: from_label (profiler . alloc_string (components . as_slice ())) }
/* FP:profiling.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_profiling_STRUCT_0008
/* FP:profiling.rs-0016 */ pub (crate) struct LlvmSelfProfiler < 'a > { profiler : Arc < SelfProfiler > , stack : Vec < TimingGuard < 'a > > , llvm_pass_event_kind : StringId , }
/* FP:profiling.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_profiling_IMPL_0009
/* FP:profiling.rs-0018 */ impl < 'a > LlvmSelfProfiler < 'a > { pub (crate) fn new (profiler : Arc < SelfProfiler >) -> Self { let llvm_pass_event_kind = profiler . alloc_string ("LLVM Pass") ; Self { profiler , stack : Vec :: default () , llvm_pass_event_kind } } fn before_pass_callback (& 'a mut self , pass_name : & str , ir_name : & str) { let event_id = llvm_args_to_string_id (& self . profiler , pass_name , ir_name) ; self . stack . push (TimingGuard :: start (& self . profiler , self . llvm_pass_event_kind , event_id)) ; } fn after_pass_callback (& mut self) { self . stack . pop () ; } }
/* FP:profiling.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_profiling_FN_0010
/* FP:profiling.rs-0020 */ pub (crate) unsafe extern "C" fn selfprofile_before_pass_callback (llvm_self_profiler : * mut c_void , pass_name : * const c_char , ir_name : * const c_char ,) { unsafe { let llvm_self_profiler = & mut * (llvm_self_profiler as * mut LlvmSelfProfiler < '_ >) ; let pass_name = CStr :: from_ptr (pass_name) . to_str () . expect ("valid UTF-8") ; let ir_name = CStr :: from_ptr (ir_name) . to_str () . expect ("valid UTF-8") ; llvm_self_profiler . before_pass_callback (pass_name , ir_name) ; } }
/* FP:profiling.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_back_profiling_FN_0011
/* FP:profiling.rs-0022 */ pub (crate) unsafe extern "C" fn selfprofile_after_pass_callback (llvm_self_profiler : * mut c_void) { let llvm_self_profiler = unsafe { & mut * (llvm_self_profiler as * mut LlvmSelfProfiler < '_ >) } ; llvm_self_profiler . after_pass_callback () ; }