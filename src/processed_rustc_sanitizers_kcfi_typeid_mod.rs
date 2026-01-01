/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_sanitizers_src_kcfi_typeid_mod_USE_0001
/* FP:mod.rs-0002 */ use std :: hash :: Hasher ;
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_sanitizers_src_kcfi_typeid_mod_USE_0002
/* FP:mod.rs-0004 */ use crate :: rustc_complete :: ty :: { Instance , InstanceKind , ReifyReason , Ty , TyCtxt } ;
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_sanitizers_src_kcfi_typeid_mod_USE_0003
/* FP:mod.rs-0006 */ use crate :: rustc_target :: callconv :: FnAbi ;
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_sanitizers_src_kcfi_typeid_mod_USE_0004
/* FP:mod.rs-0008 */ use twox_hash :: XxHash64 ;
/* FP:mod.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_sanitizers_src_kcfi_typeid_mod_USE_0005
/* FP:mod.rs-0010 */ pub use crate :: cfi :: typeid :: { TypeIdOptions , itanium_cxx_abi } ;
/* FP:mod.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_sanitizers_src_kcfi_typeid_mod_FN_0006
/* FP:mod.rs-0012 */ # [doc = " Returns a KCFI type metadata identifier for the specified FnAbi."] pub fn typeid_for_fnabi < 'tcx > (tcx : TyCtxt < 'tcx > , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > > , options : TypeIdOptions ,) -> u32 { let mut hash : XxHash64 = Default :: default () ; hash . write (itanium_cxx_abi :: typeid_for_fnabi (tcx , fn_abi , options) . as_bytes ()) ; hash . finish () as u32 }
/* FP:mod.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_sanitizers_src_kcfi_typeid_mod_FN_0007
/* FP:mod.rs-0014 */ # [doc = " Returns a KCFI type metadata identifier for the specified Instance."] pub fn typeid_for_instance < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , mut options : TypeIdOptions ,) -> u32 { if matches ! (instance . def , InstanceKind :: ReifyShim (_ , Some (ReifyReason :: FnPtr))) { options . insert (TypeIdOptions :: USE_CONCRETE_SELF) ; } let mut hash : XxHash64 = Default :: default () ; hash . write (itanium_cxx_abi :: typeid_for_instance (tcx , instance , options) . as_bytes ()) ; hash . finish () as u32 }