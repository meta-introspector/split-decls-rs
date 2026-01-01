/* FP:stable_hash_impls.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_stable_hash_impls_USE_0001
/* FP:stable_hash_impls.rs-0002 */ use crate :: rustc_data_structures :: stable_hasher :: { HashStable , StableHasher , ToStableHashKey } ;
/* FP:stable_hash_impls.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_stable_hash_impls_USE_0002
/* FP:stable_hash_impls.rs-0004 */ use crate :: rustc_complete :: def_id :: DefPathHash ;
/* FP:stable_hash_impls.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_stable_hash_impls_USE_0003
/* FP:stable_hash_impls.rs-0006 */ use crate :: HashIgnoredAttrId ;
/* FP:stable_hash_impls.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_stable_hash_impls_USE_0004
/* FP:stable_hash_impls.rs-0008 */ use crate :: hir :: { AttributeMap , BodyId , Crate , ForeignItemId , ImplItemId , ItemId , OwnerNodes , TraitItemId , } ;
/* FP:stable_hash_impls.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_stable_hash_impls_USE_0005
/* FP:stable_hash_impls.rs-0010 */ use crate :: hir_id :: ItemLocalId ;
/* FP:stable_hash_impls.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_stable_hash_impls_USE_0006
/* FP:stable_hash_impls.rs-0012 */ use crate :: lints :: DelayedLints ;
/* FP:stable_hash_impls.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_stable_hash_impls_TRAIT_0007
/* FP:stable_hash_impls.rs-0014 */ # [doc = " Requirements for a `StableHashingContext` to be used in this crate."] # [doc = " This is a hack to allow using the `HashStable_Generic` derive macro"] # [doc = " instead of implementing everything in `rustc_middle`."] pub trait HashStableContext : crate :: rustc_ast :: HashStableContext + crate :: rustc_abi :: HashStableContext { fn hash_attr_id (& mut self , id : & HashIgnoredAttrId , hasher : & mut StableHasher) ; }
/* FP:stable_hash_impls.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_stable_hash_impls_IMPL_0008
/* FP:stable_hash_impls.rs-0016 */ impl < HirCtx : crate :: HashStableContext > ToStableHashKey < HirCtx > for BodyId { type KeyType = (DefPathHash , ItemLocalId) ; # [inline] fn to_stable_hash_key (& self , hcx : & HirCtx) -> (DefPathHash , ItemLocalId) { let BodyId { hir_id } = * self ; hir_id . to_stable_hash_key (hcx) } }
/* FP:stable_hash_impls.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_stable_hash_impls_IMPL_0009
/* FP:stable_hash_impls.rs-0018 */ impl < HirCtx : crate :: HashStableContext > ToStableHashKey < HirCtx > for ItemId { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , hcx : & HirCtx) -> DefPathHash { self . owner_id . def_id . to_stable_hash_key (hcx) } }
/* FP:stable_hash_impls.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_stable_hash_impls_IMPL_0010
/* FP:stable_hash_impls.rs-0020 */ impl < HirCtx : crate :: HashStableContext > ToStableHashKey < HirCtx > for TraitItemId { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , hcx : & HirCtx) -> DefPathHash { self . owner_id . def_id . to_stable_hash_key (hcx) } }
/* FP:stable_hash_impls.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_stable_hash_impls_IMPL_0011
/* FP:stable_hash_impls.rs-0022 */ impl < HirCtx : crate :: HashStableContext > ToStableHashKey < HirCtx > for ImplItemId { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , hcx : & HirCtx) -> DefPathHash { self . owner_id . def_id . to_stable_hash_key (hcx) } }
/* FP:stable_hash_impls.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_stable_hash_impls_IMPL_0012
/* FP:stable_hash_impls.rs-0024 */ impl < HirCtx : crate :: HashStableContext > ToStableHashKey < HirCtx > for ForeignItemId { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , hcx : & HirCtx) -> DefPathHash { self . owner_id . def_id . to_stable_hash_key (hcx) } }
/* FP:stable_hash_impls.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_stable_hash_impls_IMPL_0013
/* FP:stable_hash_impls.rs-0026 */ impl < 'tcx , HirCtx : crate :: HashStableContext > HashStable < HirCtx > for OwnerNodes < 'tcx > { fn hash_stable (& self , hcx : & mut HirCtx , hasher : & mut StableHasher) { let OwnerNodes { opt_hash_including_bodies , nodes : _ , bodies : _ } = * self ; opt_hash_including_bodies . unwrap () . hash_stable (hcx , hasher) ; } }
/* FP:stable_hash_impls.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_stable_hash_impls_IMPL_0014
/* FP:stable_hash_impls.rs-0028 */ impl < HirCtx : crate :: HashStableContext > HashStable < HirCtx > for DelayedLints { fn hash_stable (& self , hcx : & mut HirCtx , hasher : & mut StableHasher) { let DelayedLints { opt_hash , .. } = * self ; opt_hash . unwrap () . hash_stable (hcx , hasher) ; } }
/* FP:stable_hash_impls.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_stable_hash_impls_IMPL_0015
/* FP:stable_hash_impls.rs-0030 */ impl < 'tcx , HirCtx : crate :: HashStableContext > HashStable < HirCtx > for AttributeMap < 'tcx > { fn hash_stable (& self , hcx : & mut HirCtx , hasher : & mut StableHasher) { let AttributeMap { opt_hash , define_opaque : _ , map : _ } = * self ; opt_hash . unwrap () . hash_stable (hcx , hasher) ; } }
/* FP:stable_hash_impls.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_stable_hash_impls_IMPL_0016
/* FP:stable_hash_impls.rs-0032 */ impl < HirCtx : crate :: HashStableContext > HashStable < HirCtx > for Crate < '_ > { fn hash_stable (& self , hcx : & mut HirCtx , hasher : & mut StableHasher) { let Crate { owners : _ , opt_hir_hash } = self ; opt_hir_hash . unwrap () . hash_stable (hcx , hasher) } }
/* FP:stable_hash_impls.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_stable_hash_impls_IMPL_0017
/* FP:stable_hash_impls.rs-0034 */ impl < HirCtx : crate :: HashStableContext > HashStable < HirCtx > for HashIgnoredAttrId { fn hash_stable (& self , hcx : & mut HirCtx , hasher : & mut StableHasher) { hcx . hash_attr_id (self , hasher) } }