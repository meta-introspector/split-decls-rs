/* FP:alloc_system.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_alloc_system_CONST_0001
/* FP:alloc_system.rs-0002 */ # [no_std] # [feature (allocator_api , rustc_private)] # [cfg (any (target_arch = "x86" , target_arch = "arm" , target_arch = "loongarch32" , target_arch = "m68k" , target_arch = "mips" , target_arch = "mips32r6" , target_arch = "powerpc" , target_arch = "csky" , target_arch = "powerpc64"))] const MIN_ALIGN : usize = 8 ;
/* FP:alloc_system.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_alloc_system_CONST_0002
/* FP:alloc_system.rs-0004 */ # [cfg (any (target_arch = "x86_64" , target_arch = "aarch64" , target_arch = "loongarch64" , target_arch = "mips64" , target_arch = "mips64r6" , target_arch = "s390x" , target_arch = "sparc64"))] const MIN_ALIGN : usize = 16 ;
/* FP:alloc_system.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_alloc_system_STRUCT_0003
/* FP:alloc_system.rs-0006 */ pub struct System ;
/* FP:alloc_system.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_alloc_system_MOD_0004
/* FP:alloc_system.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_alloc_system_MOD_0005
/* FP:alloc_system.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_alloc_system_MOD_0006