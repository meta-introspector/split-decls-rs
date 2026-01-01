/* FP:lib.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_USE_0001
/* FP:lib.rs-0002 */ # [allow (rustc :: usage_of_ty_tykind)] # [doc (html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/" , test (attr (allow (unused_variables) , deny (warnings))))] # [feature (sized_hierarchy)] use std :: fmt :: Debug ;
/* FP:lib.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_USE_0002
/* FP:lib.rs-0004 */ use std :: { fmt , io } ;
/* FP:lib.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_USE_0003
/* FP:lib.rs-0006 */ pub (crate) use crate :: rustc_public_bridge :: IndexedVal ;
/* FP:lib.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_USE_0004
/* FP:lib.rs-0008 */ use crate :: rustc_public_bridge :: Tables ;
/* FP:lib.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_USE_0005
/* FP:lib.rs-0010 */ use crate :: rustc_public_bridge :: context :: CompilerCtxt ;
/* FP:lib.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MOD_0006
/* FP:lib.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_USE_0007
/* FP:lib.rs-0014 */ use serde :: Serialize ;
/* FP:lib.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_USE_0008
/* FP:lib.rs-0016 */ use crate :: compiler_interface :: with ;
/* FP:lib.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_USE_0009
/* FP:lib.rs-0018 */ pub use crate :: crate_def :: { CrateDef , CrateDefItems , CrateDefType , DefId } ;
/* FP:lib.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_USE_0010
/* FP:lib.rs-0020 */ pub use crate :: error :: * ;
/* FP:lib.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_USE_0011
/* FP:lib.rs-0022 */ use crate :: mir :: mono :: StaticDef ;
/* FP:lib.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_USE_0012
/* FP:lib.rs-0024 */ use crate :: mir :: { Body , Mutability } ;
/* FP:lib.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_USE_0013
/* FP:lib.rs-0026 */ use crate :: ty :: { AssocItem , FnDef , ForeignModuleDef , ImplDef , ProvenanceMap , Span , TraitDef , Ty } ;
/* FP:lib.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_USE_0014
/* FP:lib.rs-0028 */ use crate :: unstable :: Stable ;
/* FP:lib.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MOD_0015
/* FP:lib.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MOD_0016
/* FP:lib.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MOD_0017
/* FP:lib.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MOD_0018
/* FP:lib.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MOD_0019
/* FP:lib.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MOD_0020
/* FP:lib.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MOD_0021
/* FP:lib.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MOD_0022
/* FP:lib.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MOD_0023
/* FP:lib.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MOD_0024
/* FP:lib.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_TYPE_0025
/* FP:lib.rs-0050 */ # [doc = " Use String for now but we should replace it."] pub type Symbol = String ;
/* FP:lib.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_TYPE_0026
/* FP:lib.rs-0052 */ # [doc = " The number that identifies a crate."] pub type CrateNum = usize ;
/* FP:lib.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_IMPL_0027
/* FP:lib.rs-0054 */ impl Debug for DefId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("DefId") . field ("id" , & self . 0) . field ("name" , & self . name ()) . finish () } }
/* FP:lib.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_IMPL_0028
/* FP:lib.rs-0056 */ impl IndexedVal for DefId { fn to_val (index : usize) -> Self { DefId (index) } fn to_index (& self) -> usize { self . 0 } }
/* FP:lib.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_TYPE_0029
/* FP:lib.rs-0058 */ # [doc = " A list of crate items."] pub type CrateItems = Vec < CrateItem > ;
/* FP:lib.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_TYPE_0030
/* FP:lib.rs-0060 */ # [doc = " A list of trait decls."] pub type TraitDecls = Vec < TraitDef > ;
/* FP:lib.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_TYPE_0031
/* FP:lib.rs-0062 */ # [doc = " A list of impl trait decls."] pub type ImplTraitDecls = Vec < ImplDef > ;
/* FP:lib.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_TYPE_0032
/* FP:lib.rs-0064 */ # [doc = " A list of associated items."] pub type AssocItems = Vec < AssocItem > ;
/* FP:lib.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_STRUCT_0033
/* FP:lib.rs-0066 */ # [doc = " Holds information about a crate."] # [derive (Clone , PartialEq , Eq , Debug , Serialize)] pub struct Crate { pub id : CrateNum , pub name : Symbol , pub is_local : bool , }
/* FP:lib.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_IMPL_0034
/* FP:lib.rs-0068 */ impl Crate { # [doc = " The list of foreign modules in this crate."] pub fn foreign_modules (& self) -> Vec < ForeignModuleDef > { with (| cx | cx . foreign_modules (self . id)) } # [doc = " The list of traits declared in this crate."] pub fn trait_decls (& self) -> TraitDecls { with (| cx | cx . trait_decls (self . id)) } # [doc = " The list of trait implementations in this crate."] pub fn trait_impls (& self) -> ImplTraitDecls { with (| cx | cx . trait_impls (self . id)) } # [doc = " Return a list of function definitions from this crate independent on their visibility."] pub fn fn_defs (& self) -> Vec < FnDef > { with (| cx | cx . crate_functions (self . id)) } # [doc = " Return a list of static items defined in this crate independent on their visibility."] pub fn statics (& self) -> Vec < StaticDef > { with (| cx | cx . crate_statics (self . id)) } }
/* FP:lib.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_ENUM_0035
/* FP:lib.rs-0070 */ # [derive (Copy , Clone , PartialEq , Eq , Debug , Hash , Serialize)] pub enum ItemKind { Fn , Static , Const , Ctor (CtorKind) , }
/* FP:lib.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_ENUM_0036
/* FP:lib.rs-0072 */ # [derive (Copy , Clone , PartialEq , Eq , Debug , Hash , Serialize)] pub enum CtorKind { Const , Fn , }
/* FP:lib.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_TYPE_0037
/* FP:lib.rs-0074 */ pub type Filename = String ;
/* FP:lib.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0038
/* FP:lib.rs-0076 */ crate_def_with_ty ! { # [doc = " Holds information about an item in a crate."] # [derive (Serialize)] pub CrateItem ; }
/* FP:lib.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_IMPL_0039
/* FP:lib.rs-0078 */ impl CrateItem { # [doc = " This will return the body of an item or panic if it's not available."] pub fn expect_body (& self) -> mir :: Body { with (| cx | cx . mir_body (self . 0)) } # [doc = " Return the body of an item if available."] pub fn body (& self) -> Option < mir :: Body > { with (| cx | cx . has_body (self . 0) . then (| | cx . mir_body (self . 0))) } # [doc = " Check if a body is available for this item."] pub fn has_body (& self) -> bool { with (| cx | cx . has_body (self . 0)) } pub fn span (& self) -> Span { with (| cx | cx . span_of_an_item (self . 0)) } pub fn kind (& self) -> ItemKind { with (| cx | cx . item_kind (* self)) } pub fn requires_monomorphization (& self) -> bool { with (| cx | cx . requires_monomorphization (self . 0)) } pub fn ty (& self) -> Ty { with (| cx | cx . def_ty (self . 0)) } pub fn is_foreign_item (& self) -> bool { with (| cx | cx . is_foreign_item (self . 0)) } # [doc = " Emit MIR for this item body."] pub fn emit_mir < W : io :: Write > (& self , w : & mut W) -> io :: Result < () > { self . body () . ok_or_else (| | io :: Error :: other (format ! ("No body found for `{}`" , self . name ()))) ? . dump (w , & self . name ()) } }
/* FP:lib.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_FN_0040
/* FP:lib.rs-0080 */ # [doc = " Return the function where execution starts if the current"] # [doc = " crate defines that. This is usually `main`, but could be"] # [doc = " `start` if the crate is a no-std crate."] pub fn entry_fn () -> Option < CrateItem > { with (| cx | cx . entry_fn ()) }
/* FP:lib.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_FN_0041
/* FP:lib.rs-0082 */ # [doc = " Access to the local crate."] pub fn local_crate () -> Crate { with (| cx | cx . local_crate ()) }
/* FP:lib.rs-0083 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_FN_0042
/* FP:lib.rs-0084 */ # [doc = " Try to find a crate or crates if multiple crates exist from given name."] pub fn find_crates (name : & str) -> Vec < Crate > { with (| cx | cx . find_crates (name)) }
/* FP:lib.rs-0085 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_FN_0043
/* FP:lib.rs-0086 */ # [doc = " Try to find a crate with the given name."] pub fn external_crates () -> Vec < Crate > { with (| cx | cx . external_crates ()) }
/* FP:lib.rs-0087 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_FN_0044
/* FP:lib.rs-0088 */ # [doc = " Retrieve all items in the local crate that have a MIR associated with them."] pub fn all_local_items () -> CrateItems { with (| cx | cx . all_local_items ()) }
/* FP:lib.rs-0089 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_FN_0045
/* FP:lib.rs-0090 */ pub fn all_trait_decls () -> TraitDecls { with (| cx | cx . all_trait_decls ()) }
/* FP:lib.rs-0091 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_FN_0046
/* FP:lib.rs-0092 */ pub fn all_trait_impls () -> ImplTraitDecls { with (| cx | cx . all_trait_impls ()) }
/* FP:lib.rs-0093 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_STRUCT_0047
/* FP:lib.rs-0094 */ # [doc = " A type that provides internal information but that can still be used for debug purpose."] # [derive (Clone , PartialEq , Eq , Hash , Serialize)] pub struct Opaque (String) ;
/* FP:lib.rs-0095 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_IMPL_0048
/* FP:lib.rs-0096 */ impl std :: fmt :: Display for Opaque { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , self . 0) } }
/* FP:lib.rs-0097 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_IMPL_0049
/* FP:lib.rs-0098 */ impl std :: fmt :: Debug for Opaque { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , self . 0) } }
/* FP:lib.rs-0099 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_FN_0050
/* FP:lib.rs-0100 */ pub fn opaque < T : Debug > (value : & T) -> Opaque { Opaque (format ! ("{value:?}")) }
/* FP:lib.rs-0101 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0051
/* FP:lib.rs-0102 */ macro_rules ! bridge_impl { ($ name : ident , $ ty : ty) => { impl crate :: rustc_public_bridge :: bridge ::$ name < compiler_interface :: BridgeTys > for $ ty { fn new (def : crate :: DefId) -> Self { Self (def) } } } ; }
/* FP:lib.rs-0103 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0052
/* FP:lib.rs-0104 */ bridge_impl ! (CrateItem , crate :: CrateItem) ;
/* FP:lib.rs-0105 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0053
/* FP:lib.rs-0106 */ bridge_impl ! (AdtDef , crate :: ty :: AdtDef) ;
/* FP:lib.rs-0107 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0054
/* FP:lib.rs-0108 */ bridge_impl ! (ForeignModuleDef , crate :: ty :: ForeignModuleDef) ;
/* FP:lib.rs-0109 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0055
/* FP:lib.rs-0110 */ bridge_impl ! (ForeignDef , crate :: ty :: ForeignDef) ;
/* FP:lib.rs-0111 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0056
/* FP:lib.rs-0112 */ bridge_impl ! (FnDef , crate :: ty :: FnDef) ;
/* FP:lib.rs-0113 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0057
/* FP:lib.rs-0114 */ bridge_impl ! (ClosureDef , crate :: ty :: ClosureDef) ;
/* FP:lib.rs-0115 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0058
/* FP:lib.rs-0116 */ bridge_impl ! (CoroutineDef , crate :: ty :: CoroutineDef) ;
/* FP:lib.rs-0117 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0059
/* FP:lib.rs-0118 */ bridge_impl ! (CoroutineClosureDef , crate :: ty :: CoroutineClosureDef) ;
/* FP:lib.rs-0119 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0060
/* FP:lib.rs-0120 */ bridge_impl ! (AliasDef , crate :: ty :: AliasDef) ;
/* FP:lib.rs-0121 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0061
/* FP:lib.rs-0122 */ bridge_impl ! (ParamDef , crate :: ty :: ParamDef) ;
/* FP:lib.rs-0123 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0062
/* FP:lib.rs-0124 */ bridge_impl ! (BrNamedDef , crate :: ty :: BrNamedDef) ;
/* FP:lib.rs-0125 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0063
/* FP:lib.rs-0126 */ bridge_impl ! (TraitDef , crate :: ty :: TraitDef) ;
/* FP:lib.rs-0127 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0064
/* FP:lib.rs-0128 */ bridge_impl ! (GenericDef , crate :: ty :: GenericDef) ;
/* FP:lib.rs-0129 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0065
/* FP:lib.rs-0130 */ bridge_impl ! (ConstDef , crate :: ty :: ConstDef) ;
/* FP:lib.rs-0131 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0066
/* FP:lib.rs-0132 */ bridge_impl ! (ImplDef , crate :: ty :: ImplDef) ;
/* FP:lib.rs-0133 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0067
/* FP:lib.rs-0134 */ bridge_impl ! (RegionDef , crate :: ty :: RegionDef) ;
/* FP:lib.rs-0135 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0068
/* FP:lib.rs-0136 */ bridge_impl ! (CoroutineWitnessDef , crate :: ty :: CoroutineWitnessDef) ;
/* FP:lib.rs-0137 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0069
/* FP:lib.rs-0138 */ bridge_impl ! (AssocDef , crate :: ty :: AssocDef) ;
/* FP:lib.rs-0139 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0070
/* FP:lib.rs-0140 */ bridge_impl ! (OpaqueDef , crate :: ty :: OpaqueDef) ;
/* FP:lib.rs-0141 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_MACRO_0071
/* FP:lib.rs-0142 */ bridge_impl ! (StaticDef , crate :: mir :: mono :: StaticDef) ;
/* FP:lib.rs-0143 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_IMPL_0072
/* FP:lib.rs-0144 */ impl crate :: rustc_public_bridge :: bridge :: Prov < compiler_interface :: BridgeTys > for crate :: ty :: Prov { fn new (aid : crate :: mir :: alloc :: AllocId) -> Self { Self (aid) } }
/* FP:lib.rs-0145 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_lib_IMPL_0073
/* FP:lib.rs-0146 */ impl crate :: rustc_public_bridge :: bridge :: Allocation < compiler_interface :: BridgeTys > for crate :: ty :: Allocation { fn new < 'tcx > (bytes : Vec < Option < u8 > > , ptrs : Vec < (usize , crate :: rustc_middle :: mir :: interpret :: AllocId) > , align : u64 , mutability : crate :: rustc_middle :: mir :: Mutability , tables : & mut Tables < 'tcx , compiler_interface :: BridgeTys > , cx : & CompilerCtxt < 'tcx , compiler_interface :: BridgeTys > ,) -> Self { Self { bytes , provenance : ProvenanceMap { ptrs : ptrs . iter () . map (| (i , aid) | (* i , tables . prov (* aid))) . collect () , } , align , mutability : mutability . stable (tables , cx) , } } }