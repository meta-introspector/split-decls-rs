/* FP:svh.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_svh_USE_0001
/* FP:svh.rs-0002 */ use std :: fmt ;
/* FP:svh.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_svh_USE_0002
/* FP:svh.rs-0004 */ use rustc_macros :: { Decodable_NoContext , Encodable_NoContext } ;
/* FP:svh.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_svh_USE_0003
/* FP:svh.rs-0006 */ use crate :: fingerprint :: Fingerprint ;
/* FP:svh.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_svh_USE_0004
/* FP:svh.rs-0008 */ use crate :: stable_hasher ;
/* FP:svh.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_svh_STRUCT_0005
/* FP:svh.rs-0010 */ # [derive (Copy , Clone , PartialEq , Eq , Debug , Encodable_NoContext , Decodable_NoContext , Hash)] pub struct Svh { hash : Fingerprint , }
/* FP:svh.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_svh_IMPL_0006
/* FP:svh.rs-0012 */ impl Svh { # [doc = " Creates a new `Svh` given the hash. If you actually want to"] # [doc = " compute the SVH from some HIR, you want the `calculate_svh`"] # [doc = " function found in `rustc_incremental`."] pub fn new (hash : Fingerprint) -> Svh { Svh { hash } } pub fn as_u128 (self) -> u128 { self . hash . as_u128 () } pub fn to_hex (self) -> String { format ! ("{:032x}" , self . hash . as_u128 ()) } }
/* FP:svh.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_svh_IMPL_0007
/* FP:svh.rs-0014 */ impl fmt :: Display for Svh { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (& self . to_hex ()) } }
/* FP:svh.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_svh_IMPL_0008
/* FP:svh.rs-0016 */ impl < T > stable_hasher :: HashStable < T > for Svh { # [inline] fn hash_stable (& self , ctx : & mut T , hasher : & mut stable_hasher :: StableHasher) { let Svh { hash } = * self ; hash . hash_stable (ctx , hasher) ; } }