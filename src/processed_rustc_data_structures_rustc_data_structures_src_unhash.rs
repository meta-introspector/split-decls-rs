/* FP:unhash.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_unhash_USE_0001
/* FP:unhash.rs-0002 */ use std :: collections :: { HashMap , HashSet } ;
/* FP:unhash.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_unhash_USE_0002
/* FP:unhash.rs-0004 */ use std :: hash :: { BuildHasherDefault , Hasher } ;
/* FP:unhash.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_unhash_TYPE_0003
/* FP:unhash.rs-0006 */ pub type UnhashMap < K , V > = HashMap < K , V , BuildHasherDefault < Unhasher > > ;
/* FP:unhash.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_unhash_TYPE_0004
/* FP:unhash.rs-0008 */ pub type UnhashSet < V > = HashSet < V , BuildHasherDefault < Unhasher > > ;
/* FP:unhash.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_unhash_TYPE_0005
/* FP:unhash.rs-0010 */ pub type UnindexMap < K , V > = indexmap :: IndexMap < K , V , BuildHasherDefault < Unhasher > > ;
/* FP:unhash.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_unhash_STRUCT_0006
/* FP:unhash.rs-0012 */ # [doc = " This no-op hasher expects only a single `write_u64` call. It's intended for"] # [doc = " map keys that already have hash-like quality, like `Fingerprint`."] # [derive (Default)] pub struct Unhasher { value : u64 , }
/* FP:unhash.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_unhash_IMPL_0007
/* FP:unhash.rs-0014 */ impl Hasher for Unhasher { # [inline] fn finish (& self) -> u64 { self . value } fn write (& mut self , _bytes : & [u8]) { unimplemented ! ("use write_u64") ; } # [inline] fn write_u64 (& mut self , value : u64) { debug_assert_eq ! (0 , self . value , "Unhasher doesn't mix values!") ; self . value = value ; } }