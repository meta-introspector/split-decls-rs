/* FP:impls_ty.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_impls_ty_USE_0001
/* FP:impls_ty.rs-0002 */ use std :: cell :: RefCell ;
/* FP:impls_ty.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_impls_ty_USE_0002
/* FP:impls_ty.rs-0004 */ use std :: ptr ;
/* FP:impls_ty.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_impls_ty_USE_0003
/* FP:impls_ty.rs-0006 */ use crate :: rustc_data_structures :: fingerprint :: Fingerprint ;
/* FP:impls_ty.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_impls_ty_USE_0004
/* FP:impls_ty.rs-0008 */ use crate :: rustc_data_structures :: fx :: FxHashMap ;
/* FP:impls_ty.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_impls_ty_USE_0005
/* FP:impls_ty.rs-0010 */ use crate :: rustc_data_structures :: stable_hasher :: { HashStable , HashingControls , StableHasher , ToStableHashKey , } ;
/* FP:impls_ty.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_impls_ty_USE_0006
/* FP:impls_ty.rs-0012 */ use rustc_query_system :: ich :: StableHashingContext ;
/* FP:impls_ty.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_impls_ty_USE_0007
/* FP:impls_ty.rs-0014 */ use tracing :: trace ;
/* FP:impls_ty.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_impls_ty_USE_0008
/* FP:impls_ty.rs-0016 */ use crate :: middle :: region ;
/* FP:impls_ty.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_impls_ty_USE_0009
/* FP:impls_ty.rs-0018 */ use crate :: { mir , ty } ;
/* FP:impls_ty.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_impls_ty_IMPL_0010
/* FP:impls_ty.rs-0020 */ impl < 'a , 'tcx , H , T > HashStable < StableHashingContext < 'a > > for & 'tcx ty :: list :: RawList < H , T > where T : HashStable < StableHashingContext < 'a > > , { fn hash_stable (& self , hcx : & mut StableHashingContext < 'a > , hasher : & mut StableHasher) { thread_local ! { static CACHE : RefCell < FxHashMap < (* const () , HashingControls) , Fingerprint >> = RefCell :: new (Default :: default ()) ; } let hash = CACHE . with (| cache | { let key = (ptr :: from_ref (* self) . cast :: < () > () , hcx . hashing_controls ()) ; if let Some (& hash) = cache . borrow () . get (& key) { return hash ; } let mut hasher = StableHasher :: new () ; self [..] . hash_stable (hcx , & mut hasher) ; let hash : Fingerprint = hasher . finish () ; cache . borrow_mut () . insert (key , hash) ; hash }) ; hash . hash_stable (hcx , hasher) ; } }
/* FP:impls_ty.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_impls_ty_IMPL_0011
/* FP:impls_ty.rs-0022 */ impl < 'a , 'tcx , H , T > ToStableHashKey < StableHashingContext < 'a > > for & 'tcx ty :: list :: RawList < H , T > where T : HashStable < StableHashingContext < 'a > > , { type KeyType = Fingerprint ; # [inline] fn to_stable_hash_key (& self , hcx : & StableHashingContext < 'a >) -> Fingerprint { let mut hasher = StableHasher :: new () ; let mut hcx : StableHashingContext < 'a > = hcx . clone () ; self . hash_stable (& mut hcx , & mut hasher) ; hasher . finish () } }
/* FP:impls_ty.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_impls_ty_IMPL_0012
/* FP:impls_ty.rs-0024 */ impl < 'a , 'tcx > HashStable < StableHashingContext < 'a > > for ty :: GenericArg < 'tcx > { fn hash_stable (& self , hcx : & mut StableHashingContext < 'a > , hasher : & mut StableHasher) { self . kind () . hash_stable (hcx , hasher) ; } }
/* FP:impls_ty.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_impls_ty_IMPL_0013
/* FP:impls_ty.rs-0026 */ impl < 'a > HashStable < StableHashingContext < 'a > > for mir :: interpret :: AllocId { fn hash_stable (& self , hcx : & mut StableHashingContext < 'a > , hasher : & mut StableHasher) { ty :: tls :: with_opt (| tcx | { trace ! ("hashing {:?}" , * self) ; let tcx = tcx . expect ("can't hash AllocIds during hir lowering") ; tcx . try_get_global_alloc (* self) . hash_stable (hcx , hasher) ; }) ; } }
/* FP:impls_ty.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_impls_ty_IMPL_0014
/* FP:impls_ty.rs-0028 */ impl < 'a > HashStable < StableHashingContext < 'a > > for mir :: interpret :: CtfeProvenance { fn hash_stable (& self , hcx : & mut StableHashingContext < 'a > , hasher : & mut StableHasher) { self . into_parts () . hash_stable (hcx , hasher) ; } }
/* FP:impls_ty.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_impls_ty_IMPL_0015
/* FP:impls_ty.rs-0030 */ impl < 'a > ToStableHashKey < StableHashingContext < 'a > > for region :: Scope { type KeyType = region :: Scope ; # [inline] fn to_stable_hash_key (& self , _ : & StableHashingContext < 'a >) -> region :: Scope { * self } }