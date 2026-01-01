/* FP:misc.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_misc_USE_0001
/* FP:misc.rs-0002 */ use std :: cell :: RefCell ;
/* FP:misc.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_misc_USE_0002
/* FP:misc.rs-0004 */ use crate :: rustc_data_structures :: fx :: FxHashMap ;
/* FP:misc.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_misc_USE_0003
/* FP:misc.rs-0006 */ use crate :: rustc_complete :: ty :: { self , Instance , Ty } ;
/* FP:misc.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_misc_USE_0004
/* FP:misc.rs-0008 */ use crate :: rustc_complete :: Session ;
/* FP:misc.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_misc_USE_0005
/* FP:misc.rs-0010 */ use super :: BackendTypes ;
/* FP:misc.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_misc_TRAIT_0006
/* FP:misc.rs-0012 */ pub trait MiscCodegenMethods < 'tcx > : BackendTypes { fn vtables (& self ,) -> & RefCell < FxHashMap < (Ty < 'tcx > , Option < ty :: ExistentialTraitRef < 'tcx > >) , Self :: Value > > ; fn apply_vcall_visibility_metadata (& self , _ty : Ty < 'tcx > , _poly_trait_ref : Option < ty :: ExistentialTraitRef < 'tcx > > , _vtable : Self :: Value ,) { } fn get_fn (& self , instance : Instance < 'tcx >) -> Self :: Function ; fn get_fn_addr (& self , instance : Instance < 'tcx >) -> Self :: Value ; fn eh_personality (& self) -> Self :: Function ; fn sess (& self) -> & Session ; fn set_frame_pointer_type (& self , llfn : Self :: Function) ; fn apply_target_cpu_attr (& self , llfn : Self :: Function) ; # [doc = " Declares the extern \"C\" main function for the entry point. Returns None if the symbol"] # [doc = " already exists."] fn declare_c_main (& self , fn_type : Self :: Type) -> Option < Self :: Function > ; }