/* FP:def_path_hash_map.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_def_path_hash_map_USE_0001
/* FP:def_path_hash_map.rs-0002 */ use rustc_hashes :: Hash64 ;
/* FP:def_path_hash_map.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_def_path_hash_map_USE_0002
/* FP:def_path_hash_map.rs-0004 */ use crate :: rustc_complete :: def_id :: DefIndex ;
/* FP:def_path_hash_map.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_def_path_hash_map_STRUCT_0003
/* FP:def_path_hash_map.rs-0006 */ # [derive (Clone , Default)] pub struct Config ;
/* FP:def_path_hash_map.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_def_path_hash_map_IMPL_0004
/* FP:def_path_hash_map.rs-0008 */ impl odht :: Config for Config { type Key = Hash64 ; type Value = DefIndex ; type EncodedKey = [u8 ; 8] ; type EncodedValue = [u8 ; 4] ; type H = odht :: UnHashFn ; # [inline] fn encode_key (k : & Hash64) -> [u8 ; 8] { k . as_u64 () . to_le_bytes () } # [inline] fn encode_value (v : & DefIndex) -> [u8 ; 4] { v . as_u32 () . to_le_bytes () } # [inline] fn decode_key (k : & [u8 ; 8]) -> Hash64 { Hash64 :: new (u64 :: from_le_bytes (* k)) } # [inline] fn decode_value (v : & [u8 ; 4]) -> DefIndex { DefIndex :: from_u32 (u32 :: from_le_bytes (* v)) } }
/* FP:def_path_hash_map.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_def_path_hash_map_TYPE_0005
/* FP:def_path_hash_map.rs-0010 */ pub type DefPathHashMap = odht :: HashTableOwned < Config > ;