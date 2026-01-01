/* FP:opaque_ty.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_opaque_ty_USE_0001
/* FP:opaque_ty.rs-0002 */ use derive_where :: derive_where ;
/* FP:opaque_ty.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_opaque_ty_USE_0002
/* FP:opaque_ty.rs-0004 */ # [cfg (feature = "nightly")] use rustc_macros :: { Decodable_NoContext , Encodable_NoContext , HashStable_NoContext } ;
/* FP:opaque_ty.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_opaque_ty_USE_0003
/* FP:opaque_ty.rs-0006 */ use rustc_type_ir_macros :: { TypeFoldable_Generic , TypeVisitable_Generic } ;
/* FP:opaque_ty.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_opaque_ty_USE_0004
/* FP:opaque_ty.rs-0008 */ use crate :: inherent :: * ;
/* FP:opaque_ty.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_opaque_ty_USE_0005
/* FP:opaque_ty.rs-0010 */ use crate :: { self as ty , Interner } ;
/* FP:opaque_ty.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_opaque_ty_STRUCT_0006
/* FP:opaque_ty.rs-0012 */ # [derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub struct OpaqueTypeKey < I : Interner > { pub def_id : I :: LocalDefId , pub args : I :: GenericArgs , }
/* FP:opaque_ty.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_opaque_ty_IMPL_0007
/* FP:opaque_ty.rs-0014 */ impl < I : Interner > Eq for OpaqueTypeKey < I > { }
/* FP:opaque_ty.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_type_ir_src_opaque_ty_IMPL_0008
/* FP:opaque_ty.rs-0016 */ impl < I : Interner > OpaqueTypeKey < I > { pub fn iter_captured_args (self , cx : I) -> impl Iterator < Item = (usize , I :: GenericArg) > { let variances = cx . variances_of (self . def_id . into ()) ; std :: iter :: zip (self . args . iter () , variances . iter ()) . enumerate () . filter_map (| (i , (arg , v)) | match (arg . kind () , v) { (_ , ty :: Invariant) => Some ((i , arg)) , (ty :: GenericArgKind :: Lifetime (_) , ty :: Bivariant) => None , _ => panic ! ("unexpected opaque type arg variance") , } ,) } pub fn fold_captured_lifetime_args (self , cx : I , mut f : impl FnMut (I :: Region) -> I :: Region ,) -> Self { let Self { def_id , args } = self ; let variances = cx . variances_of (def_id . into ()) ; let args = std :: iter :: zip (args . iter () , variances . iter ()) . map (| (arg , v) | match (arg . kind () , v) { (ty :: GenericArgKind :: Lifetime (_) , ty :: Bivariant) => arg , (ty :: GenericArgKind :: Lifetime (lt) , _) => f (lt) . into () , _ => arg , }) ; let args = cx . mk_args_from_iter (args) ; Self { def_id , args } } }