/* FP:packed.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_packed_USE_0001
/* FP:packed.rs-0002 */ use std :: cmp :: Ordering ;
/* FP:packed.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_packed_USE_0002
/* FP:packed.rs-0004 */ use std :: fmt ;
/* FP:packed.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_packed_USE_0003
/* FP:packed.rs-0006 */ use crate :: rustc_serialize :: { Decodable , Decoder , Encodable , Encoder } ;
/* FP:packed.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_packed_USE_0004
/* FP:packed.rs-0008 */ use crate :: stable_hasher :: { HashStable , StableHasher } ;
/* FP:packed.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_packed_STRUCT_0005
/* FP:packed.rs-0010 */ # [doc = " A packed 128-bit integer. Useful for reducing the size of structures in"] # [doc = " some cases."] # [derive (Copy , Clone , Debug , Hash , PartialEq , Eq , PartialOrd , Ord)] # [repr (packed (8))] pub struct Pu128 (pub u128) ;
/* FP:packed.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_packed_IMPL_0006
/* FP:packed.rs-0012 */ impl Pu128 { # [inline] pub fn get (self) -> u128 { self . 0 } }
/* FP:packed.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_packed_IMPL_0007
/* FP:packed.rs-0014 */ impl From < Pu128 > for u128 { # [inline] fn from (value : Pu128) -> Self { value . get () } }
/* FP:packed.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_packed_IMPL_0008
/* FP:packed.rs-0016 */ impl From < u128 > for Pu128 { # [inline] fn from (value : u128) -> Self { Self (value) } }
/* FP:packed.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_packed_IMPL_0009
/* FP:packed.rs-0018 */ impl PartialEq < u128 > for Pu128 { # [inline] fn eq (& self , other : & u128) -> bool { ({ self . 0 }) == * other } }
/* FP:packed.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_packed_IMPL_0010
/* FP:packed.rs-0020 */ impl PartialOrd < u128 > for Pu128 { # [inline] fn partial_cmp (& self , other : & u128) -> Option < Ordering > { { self . 0 } . partial_cmp (other) } }
/* FP:packed.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_packed_IMPL_0011
/* FP:packed.rs-0022 */ impl fmt :: Display for Pu128 { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { { self . 0 } . fmt (f) } }
/* FP:packed.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_packed_IMPL_0012
/* FP:packed.rs-0024 */ impl fmt :: UpperHex for Pu128 { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { { self . 0 } . fmt (f) } }
/* FP:packed.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_packed_IMPL_0013
/* FP:packed.rs-0026 */ impl < CTX > HashStable < CTX > for Pu128 { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { { self . 0 } . hash_stable (ctx , hasher) } }
/* FP:packed.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_packed_IMPL_0014
/* FP:packed.rs-0028 */ impl < S : Encoder > Encodable < S > for Pu128 { # [inline] fn encode (& self , s : & mut S) { { self . 0 } . encode (s) ; } }
/* FP:packed.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_packed_IMPL_0015
/* FP:packed.rs-0030 */ impl < D : Decoder > Decodable < D > for Pu128 { # [inline] fn decode (d : & mut D) -> Self { Self (u128 :: decode (d)) } }