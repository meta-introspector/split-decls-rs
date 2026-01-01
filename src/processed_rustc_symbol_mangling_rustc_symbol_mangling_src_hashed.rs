/* FP:hashed.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_hashed_USE_0001
/* FP:hashed.rs-0002 */ use std :: fmt :: Write ;
/* FP:hashed.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_hashed_USE_0002
/* FP:hashed.rs-0004 */ use crate :: rustc_data_structures :: stable_hasher :: { HashStable , StableHasher } ;
/* FP:hashed.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_hashed_USE_0003
/* FP:hashed.rs-0006 */ use rustc_hashes :: Hash64 ;
/* FP:hashed.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_hashed_USE_0004
/* FP:hashed.rs-0008 */ use crate :: rustc_complete :: def_id :: CrateNum ;
/* FP:hashed.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_hashed_USE_0005
/* FP:hashed.rs-0010 */ use crate :: rustc_complete :: ty :: { Instance , TyCtxt } ;
/* FP:hashed.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_hashed_USE_0006
/* FP:hashed.rs-0012 */ use crate :: v0 ;
/* FP:hashed.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_hashed_FN_0007
/* FP:hashed.rs-0014 */ pub (super) fn mangle < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , instantiating_crate : Option < CrateNum > , full_mangling_name : impl FnOnce () -> String ,) -> String { let crate_num = if let Some (krate) = instantiating_crate { krate } else { instance . def_id () . krate } ; let mut symbol = "_RNxC" . to_string () ; v0 :: push_ident (tcx . crate_name (crate_num) . as_str () , & mut symbol) ; let hash = tcx . with_stable_hashing_context (| mut hcx | { let mut hasher = StableHasher :: new () ; full_mangling_name () . hash_stable (& mut hcx , & mut hasher) ; hasher . finish :: < Hash64 > () . as_u64 () }) ; push_hash64 (hash , & mut symbol) ; symbol }
/* FP:hashed.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_hashed_FN_0008
/* FP:hashed.rs-0016 */ fn push_hash64 (hash : u64 , output : & mut String) { let hash = v0 :: encode_integer_62 (hash) ; let hash_len = hash . len () ; let _ = write ! (output , "{hash_len}H{}" , & hash [.. hash_len - 1]) ; }