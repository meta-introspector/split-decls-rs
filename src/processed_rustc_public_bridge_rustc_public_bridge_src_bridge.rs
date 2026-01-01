/* FP:bridge.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_USE_0001
/* FP:bridge.rs-0002 */ use std :: fmt :: Debug ;
/* FP:bridge.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_USE_0002
/* FP:bridge.rs-0004 */ use super :: context :: CompilerCtxt ;
/* FP:bridge.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_USE_0003
/* FP:bridge.rs-0006 */ use super :: { Bridge , Tables } ;
/* FP:bridge.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_TRAIT_0004
/* FP:bridge.rs-0008 */ pub trait Error { fn new (msg : String) -> Self ; fn from_internal < T : Debug > (err : T) -> Self ; }
/* FP:bridge.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_TRAIT_0005
/* FP:bridge.rs-0010 */ pub trait Prov < B : Bridge > { fn new (aid : B :: AllocId) -> Self ; }
/* FP:bridge.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_TRAIT_0006
/* FP:bridge.rs-0012 */ pub trait Allocation < B : Bridge > { fn new < 'tcx > (bytes : Vec < Option < u8 > > , ptrs : Vec < (usize , crate :: rustc_middle :: mir :: interpret :: AllocId) > , align : u64 , mutability : crate :: rustc_middle :: mir :: Mutability , tables : & mut Tables < 'tcx , B > , cx : & CompilerCtxt < 'tcx , B > ,) -> Self ; }
/* FP:bridge.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_MACRO_0007
/* FP:bridge.rs-0014 */ macro_rules ! make_bridge_trait { ($ name : ident) => { pub trait $ name < B : Bridge > { fn new (did : B :: DefId) -> Self ; } } ; }
/* FP:bridge.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_MACRO_0008
/* FP:bridge.rs-0016 */ make_bridge_trait ! (CrateItem) ;
/* FP:bridge.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_MACRO_0009
/* FP:bridge.rs-0018 */ make_bridge_trait ! (AdtDef) ;
/* FP:bridge.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_MACRO_0010
/* FP:bridge.rs-0020 */ make_bridge_trait ! (ForeignModuleDef) ;
/* FP:bridge.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_MACRO_0011
/* FP:bridge.rs-0022 */ make_bridge_trait ! (ForeignDef) ;
/* FP:bridge.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_MACRO_0012
/* FP:bridge.rs-0024 */ make_bridge_trait ! (FnDef) ;
/* FP:bridge.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_MACRO_0013
/* FP:bridge.rs-0026 */ make_bridge_trait ! (ClosureDef) ;
/* FP:bridge.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_MACRO_0014
/* FP:bridge.rs-0028 */ make_bridge_trait ! (CoroutineDef) ;
/* FP:bridge.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_MACRO_0015
/* FP:bridge.rs-0030 */ make_bridge_trait ! (CoroutineClosureDef) ;
/* FP:bridge.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_MACRO_0016
/* FP:bridge.rs-0032 */ make_bridge_trait ! (AliasDef) ;
/* FP:bridge.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_MACRO_0017
/* FP:bridge.rs-0034 */ make_bridge_trait ! (ParamDef) ;
/* FP:bridge.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_MACRO_0018
/* FP:bridge.rs-0036 */ make_bridge_trait ! (BrNamedDef) ;
/* FP:bridge.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_MACRO_0019
/* FP:bridge.rs-0038 */ make_bridge_trait ! (TraitDef) ;
/* FP:bridge.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_MACRO_0020
/* FP:bridge.rs-0040 */ make_bridge_trait ! (GenericDef) ;
/* FP:bridge.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_MACRO_0021
/* FP:bridge.rs-0042 */ make_bridge_trait ! (ConstDef) ;
/* FP:bridge.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_MACRO_0022
/* FP:bridge.rs-0044 */ make_bridge_trait ! (ImplDef) ;
/* FP:bridge.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_MACRO_0023
/* FP:bridge.rs-0046 */ make_bridge_trait ! (RegionDef) ;
/* FP:bridge.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_MACRO_0024
/* FP:bridge.rs-0048 */ make_bridge_trait ! (CoroutineWitnessDef) ;
/* FP:bridge.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_MACRO_0025
/* FP:bridge.rs-0050 */ make_bridge_trait ! (AssocDef) ;
/* FP:bridge.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_MACRO_0026
/* FP:bridge.rs-0052 */ make_bridge_trait ! (OpaqueDef) ;
/* FP:bridge.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_bridge_src_bridge_MACRO_0027
/* FP:bridge.rs-0054 */ make_bridge_trait ! (StaticDef) ;