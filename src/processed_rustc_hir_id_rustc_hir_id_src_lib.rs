/* FP:lib.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_USE_0001
/* FP:lib.rs-0002 */ # [allow (internal_features)] # [feature (negative_impls)] # [feature (rustc_attrs)] use std :: fmt :: { self , Debug } ;
/* FP:lib.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_USE_0002
/* FP:lib.rs-0004 */ use crate :: rustc_data_structures :: stable_hasher :: { HashStable , StableHasher , StableOrd , ToStableHashKey } ;
/* FP:lib.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_USE_0003
/* FP:lib.rs-0006 */ use rustc_macros :: { Decodable , Encodable , HashStable_Generic } ;
/* FP:lib.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_USE_0004
/* FP:lib.rs-0008 */ pub use crate :: rustc_complete :: HashStableContext ;
/* FP:lib.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_USE_0005
/* FP:lib.rs-0010 */ use crate :: rustc_complete :: def_id :: { CRATE_DEF_ID , DefId , DefIndex , DefPathHash , LocalDefId } ;
/* FP:lib.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_STRUCT_0006
/* FP:lib.rs-0012 */ # [derive (Copy , Clone , PartialEq , Eq , Hash , Encodable , Decodable)] pub struct OwnerId { pub def_id : LocalDefId , }
/* FP:lib.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_IMPL_0007
/* FP:lib.rs-0014 */ impl Debug for OwnerId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { Debug :: fmt (& self . def_id , f) } }
/* FP:lib.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_IMPL_0008
/* FP:lib.rs-0016 */ impl From < OwnerId > for HirId { fn from (owner : OwnerId) -> HirId { HirId { owner , local_id : ItemLocalId :: ZERO } } }
/* FP:lib.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_IMPL_0009
/* FP:lib.rs-0018 */ impl From < OwnerId > for DefId { fn from (value : OwnerId) -> Self { value . to_def_id () } }
/* FP:lib.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_IMPL_0010
/* FP:lib.rs-0020 */ impl OwnerId { # [inline] pub fn to_def_id (self) -> DefId { self . def_id . to_def_id () } }
/* FP:lib.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_IMPL_0011
/* FP:lib.rs-0022 */ impl crate :: rustc_index :: Idx for OwnerId { # [inline] fn new (idx : usize) -> Self { OwnerId { def_id : LocalDefId { local_def_index : DefIndex :: from_usize (idx) } } } # [inline] fn index (self) -> usize { self . def_id . local_def_index . as_usize () } }
/* FP:lib.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_IMPL_0012
/* FP:lib.rs-0024 */ impl < CTX : HashStableContext > HashStable < CTX > for OwnerId { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . to_stable_hash_key (hcx) . hash_stable (hcx , hasher) ; } }
/* FP:lib.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_IMPL_0013
/* FP:lib.rs-0026 */ impl < CTX : HashStableContext > ToStableHashKey < CTX > for OwnerId { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , hcx : & CTX) -> DefPathHash { hcx . def_path_hash (self . to_def_id ()) } }
/* FP:lib.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_STRUCT_0014
/* FP:lib.rs-0028 */ # [doc = " Uniquely identifies a node in the HIR of the current crate. It is"] # [doc = " composed of the `owner`, which is the `LocalDefId` of the directly enclosing"] # [doc = " `hir::Item`, `hir::TraitItem`, or `hir::ImplItem` (i.e., the closest \"item-like\"),"] # [doc = " and the `local_id` which is unique within the given owner."] # [doc = ""] # [doc = " This two-level structure makes for more stable values: One can move an item"] # [doc = " around within the source code, or add or remove stuff before it, without"] # [doc = " the `local_id` part of the `HirId` changing, which is a very useful property in"] # [doc = " incremental compilation where we have to persist things through changes to"] # [doc = " the code base."] # [derive (Copy , Clone , PartialEq , Eq , Hash , Encodable , Decodable , HashStable_Generic)] # [rustc_pass_by_value] pub struct HirId { pub owner : OwnerId , pub local_id : ItemLocalId , }
/* FP:lib.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_IMPL_0015
/* FP:lib.rs-0030 */ impl ! Ord for HirId { }
/* FP:lib.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_IMPL_0016
/* FP:lib.rs-0032 */ impl ! PartialOrd for HirId { }
/* FP:lib.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_IMPL_0017
/* FP:lib.rs-0034 */ impl Debug for HirId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "HirId({:?}.{:?})" , self . owner , self . local_id) } }
/* FP:lib.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_IMPL_0018
/* FP:lib.rs-0036 */ impl HirId { # [doc = " Signal local id which should never be used."] pub const INVALID : HirId = HirId { owner : OwnerId { def_id : CRATE_DEF_ID } , local_id : ItemLocalId :: INVALID } ; # [inline] pub fn expect_owner (self) -> OwnerId { assert_eq ! (self . local_id . index () , 0) ; self . owner } # [inline] pub fn as_owner (self) -> Option < OwnerId > { if self . local_id . index () == 0 { Some (self . owner) } else { None } } # [inline] pub fn is_owner (self) -> bool { self . local_id . index () == 0 } # [inline] pub fn make_owner (owner : LocalDefId) -> Self { Self { owner : OwnerId { def_id : owner } , local_id : ItemLocalId :: ZERO } } }
/* FP:lib.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_IMPL_0019
/* FP:lib.rs-0038 */ impl fmt :: Display for HirId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{self:?}") } }
/* FP:lib.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_MACRO_0020
/* FP:lib.rs-0040 */ crate :: rustc_data_structures :: define_stable_id_collections ! (HirIdMap , HirIdSet , HirIdMapEntry , HirId) ;
/* FP:lib.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_MACRO_0021
/* FP:lib.rs-0042 */ crate :: rustc_data_structures :: define_id_collections ! (ItemLocalMap , ItemLocalSet , ItemLocalMapEntry , ItemLocalId) ;
/* FP:lib.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_MACRO_0022
/* FP:lib.rs-0044 */ crate :: rustc_index :: newtype_index ! { # [doc = " An `ItemLocalId` uniquely identifies something within a given \"item-like\";"] # [doc = " that is, within a `hir::Item`, `hir::TraitItem`, or `hir::ImplItem`. There is no"] # [doc = " guarantee that the numerical value of a given `ItemLocalId` corresponds to"] # [doc = " the node's position within the owning item in any way, but there is a"] # [doc = " guarantee that the `ItemLocalId`s within an owner occupy a dense range of"] # [doc = " integers starting at zero, so a mapping that maps all or most nodes within"] # [doc = " an \"item-like\" to something else can be implemented by a `Vec` instead of a"] # [doc = " tree or hash map."] # [derive (HashStable_Generic)] # [encodable] # [orderable] pub struct ItemLocalId { } }
/* FP:lib.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_IMPL_0023
/* FP:lib.rs-0046 */ impl ItemLocalId { # [doc = " Signal local id which should never be used."] pub const INVALID : ItemLocalId = ItemLocalId :: MAX ; }
/* FP:lib.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_IMPL_0024
/* FP:lib.rs-0048 */ impl StableOrd for ItemLocalId { const CAN_USE_UNSTABLE_SORT : bool = true ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }
/* FP:lib.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_CONST_0025
/* FP:lib.rs-0050 */ # [doc = " The `HirId` corresponding to `CRATE_NODE_ID` and `CRATE_DEF_ID`."] pub const CRATE_HIR_ID : HirId = HirId { owner : OwnerId { def_id : CRATE_DEF_ID } , local_id : ItemLocalId :: ZERO } ;
/* FP:lib.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_CONST_0026
/* FP:lib.rs-0052 */ pub const CRATE_OWNER_ID : OwnerId = OwnerId { def_id : CRATE_DEF_ID } ;
/* FP:lib.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_IMPL_0027
/* FP:lib.rs-0054 */ impl < CTX : crate :: rustc_span :: HashStableContext > ToStableHashKey < CTX > for HirId { type KeyType = (DefPathHash , ItemLocalId) ; # [inline] fn to_stable_hash_key (& self , hcx : & CTX) -> (DefPathHash , ItemLocalId) { let def_path_hash = self . owner . def_id . to_stable_hash_key (hcx) ; (def_path_hash , self . local_id) } }
/* FP:lib.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_id_src_lib_IMPL_0028
/* FP:lib.rs-0056 */ impl < CTX : HashStableContext > ToStableHashKey < CTX > for ItemLocalId { type KeyType = ItemLocalId ; # [inline] fn to_stable_hash_key (& self , _ : & CTX) -> ItemLocalId { * self } }