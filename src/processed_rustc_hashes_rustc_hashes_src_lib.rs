/* FP:lib.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hashes_src_lib_USE_0001
/* FP:lib.rs-0002 */ use std :: fmt ;
/* FP:lib.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hashes_src_lib_USE_0002
/* FP:lib.rs-0004 */ use std :: ops :: BitXorAssign ;
/* FP:lib.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hashes_src_lib_USE_0003
/* FP:lib.rs-0006 */ use rustc_stable_hash :: { FromStableHash , SipHasher128Hash as StableHasherHash } ;
/* FP:lib.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hashes_src_lib_STRUCT_0004
/* FP:lib.rs-0008 */ # [doc = " A `u64` but encoded with a fixed size; for hashes this encoding is more compact than `u64`."] # [derive (Clone , Copy , PartialEq , Eq , Hash , PartialOrd , Ord , Default)] pub struct Hash64 { inner : u64 , }
/* FP:lib.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hashes_src_lib_IMPL_0005
/* FP:lib.rs-0010 */ impl Hash64 { pub const ZERO : Hash64 = Hash64 { inner : 0 } ; # [inline] pub fn new (n : u64) -> Self { Self { inner : n } } # [inline] pub fn as_u64 (self) -> u64 { self . inner } # [inline] pub fn wrapping_add (self , other : Self) -> Self { Self { inner : self . inner . wrapping_add (other . inner) } } }
/* FP:lib.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hashes_src_lib_IMPL_0006
/* FP:lib.rs-0012 */ impl BitXorAssign < u64 > for Hash64 { # [inline] fn bitxor_assign (& mut self , rhs : u64) { self . inner ^= rhs ; } }
/* FP:lib.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hashes_src_lib_IMPL_0007
/* FP:lib.rs-0014 */ impl FromStableHash for Hash64 { type Hash = StableHasherHash ; # [inline] fn from (StableHasherHash ([_0 , __1]) : Self :: Hash) -> Self { Self { inner : _0 } } }
/* FP:lib.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hashes_src_lib_IMPL_0008
/* FP:lib.rs-0016 */ impl fmt :: Debug for Hash64 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . inner . fmt (f) } }
/* FP:lib.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hashes_src_lib_IMPL_0009
/* FP:lib.rs-0018 */ impl fmt :: LowerHex for Hash64 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (& self . inner , f) } }
/* FP:lib.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hashes_src_lib_STRUCT_0010
/* FP:lib.rs-0020 */ # [doc = " A `u128` but encoded with a fixed size; for hashes this encoding is more compact than `u128`."] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Default)] pub struct Hash128 { inner : u128 , }
/* FP:lib.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hashes_src_lib_IMPL_0011
/* FP:lib.rs-0022 */ impl std :: hash :: Hash for Hash128 { fn hash < H : std :: hash :: Hasher > (& self , h : & mut H) { h . write_u64 (self . truncate () . as_u64 ()) ; } }
/* FP:lib.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hashes_src_lib_IMPL_0012
/* FP:lib.rs-0024 */ impl Hash128 { # [inline] pub fn new (n : u128) -> Self { Self { inner : n } } # [inline] pub fn truncate (self) -> Hash64 { Hash64 { inner : self . inner as u64 } } # [inline] pub fn wrapping_add (self , other : Self) -> Self { Self { inner : self . inner . wrapping_add (other . inner) } } # [inline] pub fn as_u128 (self) -> u128 { self . inner } }
/* FP:lib.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hashes_src_lib_IMPL_0013
/* FP:lib.rs-0026 */ impl FromStableHash for Hash128 { type Hash = StableHasherHash ; # [inline] fn from (StableHasherHash ([_0 , _1]) : Self :: Hash) -> Self { Self { inner : u128 :: from (_0) | (u128 :: from (_1) << 64) } } }
/* FP:lib.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hashes_src_lib_IMPL_0014
/* FP:lib.rs-0028 */ impl fmt :: Debug for Hash128 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . inner . fmt (f) } }
/* FP:lib.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hashes_src_lib_IMPL_0015
/* FP:lib.rs-0030 */ impl fmt :: LowerHex for Hash128 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (& self . inner , f) } }