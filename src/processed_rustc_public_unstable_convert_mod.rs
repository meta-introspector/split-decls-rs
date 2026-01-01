/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_unstable_convert_mod_USE_0001
/* FP:mod.rs-0002 */ use std :: ops :: RangeInclusive ;
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_unstable_convert_mod_USE_0002
/* FP:mod.rs-0004 */ use crate :: rustc_public_bridge :: Tables ;
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_unstable_convert_mod_USE_0003
/* FP:mod.rs-0006 */ use crate :: rustc_public_bridge :: context :: CompilerCtxt ;
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_unstable_convert_mod_USE_0004
/* FP:mod.rs-0008 */ use super :: Stable ;
/* FP:mod.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_unstable_convert_mod_USE_0005
/* FP:mod.rs-0010 */ use crate :: compiler_interface :: BridgeTys ;
/* FP:mod.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_unstable_convert_mod_MOD_0006
/* FP:mod.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_unstable_convert_mod_MOD_0007
/* FP:mod.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_unstable_convert_mod_IMPL_0008
/* FP:mod.rs-0016 */ impl < 'tcx , T > Stable < 'tcx > for & T where T : Stable < 'tcx > , { type T = T :: T ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { (* self) . stable (tables , cx) } }
/* FP:mod.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_unstable_convert_mod_IMPL_0009
/* FP:mod.rs-0018 */ impl < 'tcx , T > Stable < 'tcx > for Option < T > where T : Stable < 'tcx > , { type T = Option < T :: T > ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { self . as_ref () . map (| value | value . stable (tables , cx)) } }
/* FP:mod.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_unstable_convert_mod_IMPL_0010
/* FP:mod.rs-0020 */ impl < 'tcx , T , E > Stable < 'tcx > for Result < T , E > where T : Stable < 'tcx > , E : Stable < 'tcx > , { type T = Result < T :: T , E :: T > ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { match self { Ok (val) => Ok (val . stable (tables , cx)) , Err (error) => Err (error . stable (tables , cx)) , } } }
/* FP:mod.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_unstable_convert_mod_IMPL_0011
/* FP:mod.rs-0022 */ impl < 'tcx , T > Stable < 'tcx > for & [T] where T : Stable < 'tcx > , { type T = Vec < T :: T > ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { self . iter () . map (| e | e . stable (tables , cx)) . collect () } }
/* FP:mod.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_unstable_convert_mod_IMPL_0012
/* FP:mod.rs-0024 */ impl < 'tcx , T , U > Stable < 'tcx > for (T , U) where T : Stable < 'tcx > , U : Stable < 'tcx > , { type T = (T :: T , U :: T) ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { (self . 0 . stable (tables , cx) , self . 1 . stable (tables , cx)) } }
/* FP:mod.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_unstable_convert_mod_IMPL_0013
/* FP:mod.rs-0026 */ impl < 'tcx , T > Stable < 'tcx > for RangeInclusive < T > where T : Stable < 'tcx > , { type T = RangeInclusive < T :: T > ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { RangeInclusive :: new (self . start () . stable (tables , cx) , self . end () . stable (tables , cx)) } }