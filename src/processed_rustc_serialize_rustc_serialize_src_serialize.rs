/* FP:serialize.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_USE_0001
/* FP:serialize.rs-0002 */ use std :: borrow :: Cow ;
/* FP:serialize.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_USE_0002
/* FP:serialize.rs-0004 */ use std :: cell :: { Cell , RefCell } ;
/* FP:serialize.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_USE_0003
/* FP:serialize.rs-0006 */ use std :: collections :: { BTreeMap , BTreeSet , HashMap , HashSet , VecDeque } ;
/* FP:serialize.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_USE_0004
/* FP:serialize.rs-0008 */ use std :: hash :: { BuildHasher , Hash } ;
/* FP:serialize.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_USE_0005
/* FP:serialize.rs-0010 */ use std :: marker :: { PhantomData , PointeeSized } ;
/* FP:serialize.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_USE_0006
/* FP:serialize.rs-0012 */ use std :: num :: NonZero ;
/* FP:serialize.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_USE_0007
/* FP:serialize.rs-0014 */ use std :: path ;
/* FP:serialize.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_USE_0008
/* FP:serialize.rs-0016 */ use std :: rc :: Rc ;
/* FP:serialize.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_USE_0009
/* FP:serialize.rs-0018 */ use std :: sync :: Arc ;
/* FP:serialize.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_USE_0010
/* FP:serialize.rs-0020 */ use rustc_hashes :: { Hash64 , Hash128 } ;
/* FP:serialize.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_USE_0011
/* FP:serialize.rs-0022 */ use smallvec :: { Array , SmallVec } ;
/* FP:serialize.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_USE_0012
/* FP:serialize.rs-0024 */ use thin_vec :: ThinVec ;
/* FP:serialize.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_CONST_0013
/* FP:serialize.rs-0026 */ # [doc = " A byte that [cannot occur in UTF8 sequences][utf8]. Used to mark the end of a string."] # [doc = " This way we can skip validation and still be relatively sure that deserialization"] # [doc = " did not desynchronize."] # [doc = ""] # [doc = " [utf8]: https://en.wikipedia.org/w/index.php?title=UTF-8&oldid=1058865525#Codepage_layout"] const STR_SENTINEL : u8 = 0xC1 ;
/* FP:serialize.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_CONST_0014
/* FP:serialize.rs-0028 */ # [doc = " For byte strings there are no bytes that cannot occur. Just use this value"] # [doc = " as a best-effort sentinel. There is no validation skipped so the potential"] # [doc = " for badness is lower than in the `STR_SENTINEL` case."] const BYTE_STR_SENTINEL : u8 = 0xC2 ;
/* FP:serialize.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_TRAIT_0015
/* FP:serialize.rs-0030 */ # [doc = " A note about error handling."] # [doc = ""] # [doc = " Encoders may be fallible, but in practice failure is rare and there are so"] # [doc = " many nested calls that typical Rust error handling (via `Result` and `?`)"] # [doc = " is pervasive and has non-trivial cost. Instead, impls of this trait must"] # [doc = " implement a delayed error handling strategy. If a failure occurs, they"] # [doc = " should record this internally, and all subsequent encoding operations can"] # [doc = " be processed or ignored, whichever is appropriate. Then they should provide"] # [doc = " a `finish` method that finishes up encoding. If the encoder is fallible,"] # [doc = " `finish` should return a `Result` that indicates success or failure."] # [doc = ""] # [doc = " This current does not support `f32` nor `f64`, as they're not needed in any"] # [doc = " serialized data structures. That could be changed, but consider whether it"] # [doc = " really makes sense to store floating-point values at all."] # [doc = " (If you need it, revert <https://github.com/rust-lang/rust/pull/109984>.)"] pub trait Encoder { fn emit_usize (& mut self , v : usize) ; fn emit_u128 (& mut self , v : u128) ; fn emit_u64 (& mut self , v : u64) ; fn emit_u32 (& mut self , v : u32) ; fn emit_u16 (& mut self , v : u16) ; fn emit_u8 (& mut self , v : u8) ; fn emit_isize (& mut self , v : isize) ; fn emit_i128 (& mut self , v : i128) ; fn emit_i64 (& mut self , v : i64) ; fn emit_i32 (& mut self , v : i32) ; fn emit_i16 (& mut self , v : i16) ; # [inline] fn emit_i8 (& mut self , v : i8) { self . emit_u8 (v as u8) ; } # [inline] fn emit_bool (& mut self , v : bool) { self . emit_u8 (if v { 1 } else { 0 }) ; } # [inline] fn emit_char (& mut self , v : char) { self . emit_u32 (v as u32) ; } # [inline] fn emit_str (& mut self , v : & str) { self . emit_usize (v . len ()) ; self . emit_raw_bytes (v . as_bytes ()) ; self . emit_u8 (STR_SENTINEL) ; } # [inline] fn emit_byte_str (& mut self , v : & [u8]) { self . emit_usize (v . len ()) ; self . emit_raw_bytes (v) ; self . emit_u8 (BYTE_STR_SENTINEL) ; } fn emit_raw_bytes (& mut self , s : & [u8]) ; }
/* FP:serialize.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_TRAIT_0016
/* FP:serialize.rs-0032 */ # [doc = ""] # [doc = " This current does not support `f32` nor `f64`, as they're not needed in any"] # [doc = " serialized data structures. That could be changed, but consider whether it"] # [doc = " really makes sense to store floating-point values at all."] # [doc = " (If you need it, revert <https://github.com/rust-lang/rust/pull/109984>.)"] pub trait Decoder { fn read_usize (& mut self) -> usize ; fn read_u128 (& mut self) -> u128 ; fn read_u64 (& mut self) -> u64 ; fn read_u32 (& mut self) -> u32 ; fn read_u16 (& mut self) -> u16 ; fn read_u8 (& mut self) -> u8 ; fn read_isize (& mut self) -> isize ; fn read_i128 (& mut self) -> i128 ; fn read_i64 (& mut self) -> i64 ; fn read_i32 (& mut self) -> i32 ; fn read_i16 (& mut self) -> i16 ; # [inline] fn read_i8 (& mut self) -> i8 { self . read_u8 () as i8 } # [inline] fn read_bool (& mut self) -> bool { let value = self . read_u8 () ; value != 0 } # [inline] fn read_char (& mut self) -> char { let bits = self . read_u32 () ; std :: char :: from_u32 (bits) . unwrap () } # [inline] fn read_str (& mut self) -> & str { let len = self . read_usize () ; let bytes = self . read_raw_bytes (len + 1) ; assert ! (bytes [len] == STR_SENTINEL) ; unsafe { std :: str :: from_utf8_unchecked (& bytes [.. len]) } } # [inline] fn read_byte_str (& mut self) -> & [u8] { let len = self . read_usize () ; let bytes = self . read_raw_bytes (len + 1) ; assert ! (bytes [len] == BYTE_STR_SENTINEL) ; & bytes [.. len] } fn read_raw_bytes (& mut self , len : usize) -> & [u8] ; fn peek_byte (& self) -> u8 ; fn position (& self) -> usize ; }
/* FP:serialize.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_TRAIT_0017
/* FP:serialize.rs-0034 */ # [doc = " Trait for types that can be serialized"] # [doc = ""] # [doc = " This can be implemented using the `Encodable`, `TyEncodable` and"] # [doc = " `MetadataEncodable` macros."] # [doc = ""] # [doc = " * `Encodable` should be used in crates that don't depend on"] # [doc = "   `rustc_middle`."] # [doc = " * `MetadataEncodable` is used in `rustc_metadata` for types that contain"] # [doc = "   `crate::rustc_metadata::rmeta::Lazy`."] # [doc = " * `TyEncodable` should be used for types that are only serialized in crate"] # [doc = "   metadata or the incremental cache. This is most types in `rustc_middle`."] pub trait Encodable < S : Encoder > : PointeeSized { fn encode (& self , s : & mut S) ; }
/* FP:serialize.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_TRAIT_0018
/* FP:serialize.rs-0036 */ # [doc = " Trait for types that can be deserialized"] # [doc = ""] # [doc = " This can be implemented using the `Decodable`, `TyDecodable` and"] # [doc = " `MetadataDecodable` macros."] # [doc = ""] # [doc = " * `Decodable` should be used in crates that don't depend on"] # [doc = "   `rustc_middle`."] # [doc = " * `MetadataDecodable` is used in `rustc_metadata` for types that contain"] # [doc = "   `crate::rustc_metadata::rmeta::Lazy`."] # [doc = " * `TyDecodable` should be used for types that are only serialized in crate"] # [doc = "   metadata or the incremental cache. This is most types in `rustc_middle`."] pub trait Decodable < D : Decoder > : Sized { fn decode (d : & mut D) -> Self ; }
/* FP:serialize.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_MACRO_0019
/* FP:serialize.rs-0038 */ macro_rules ! direct_serialize_impls { ($ ($ ty : ident $ emit_method : ident $ read_method : ident) ,*) => { $ (impl < S : Encoder > Encodable < S > for $ ty { fn encode (& self , s : & mut S) { s .$ emit_method (* self) ; } } impl < D : Decoder > Decodable < D > for $ ty { fn decode (d : & mut D) -> $ ty { d .$ read_method () } }) * } }
/* FP:serialize.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_MACRO_0020
/* FP:serialize.rs-0040 */ direct_serialize_impls ! { usize emit_usize read_usize , u8 emit_u8 read_u8 , u16 emit_u16 read_u16 , u32 emit_u32 read_u32 , u64 emit_u64 read_u64 , u128 emit_u128 read_u128 , isize emit_isize read_isize , i8 emit_i8 read_i8 , i16 emit_i16 read_i16 , i32 emit_i32 read_i32 , i64 emit_i64 read_i64 , i128 emit_i128 read_i128 , bool emit_bool read_bool , char emit_char read_char }
/* FP:serialize.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0021
/* FP:serialize.rs-0042 */ impl < S : Encoder , T : ? Sized + PointeeSized > Encodable < S > for & T where T : Encodable < S > , { fn encode (& self , s : & mut S) { (* * self) . encode (s) } }
/* FP:serialize.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0022
/* FP:serialize.rs-0044 */ impl < S : Encoder > Encodable < S > for ! { fn encode (& self , _s : & mut S) { unreachable ! () ; } }
/* FP:serialize.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0023
/* FP:serialize.rs-0046 */ impl < D : Decoder > Decodable < D > for ! { fn decode (_d : & mut D) -> ! { unreachable ! () } }
/* FP:serialize.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0024
/* FP:serialize.rs-0048 */ impl < S : Encoder > Encodable < S > for NonZero < u32 > { fn encode (& self , s : & mut S) { s . emit_u32 (self . get ()) ; } }
/* FP:serialize.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0025
/* FP:serialize.rs-0050 */ impl < D : Decoder > Decodable < D > for NonZero < u32 > { fn decode (d : & mut D) -> Self { NonZero :: new (d . read_u32 ()) . unwrap () } }
/* FP:serialize.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0026
/* FP:serialize.rs-0052 */ impl < S : Encoder > Encodable < S > for str { fn encode (& self , s : & mut S) { s . emit_str (self) ; } }
/* FP:serialize.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0027
/* FP:serialize.rs-0054 */ impl < S : Encoder > Encodable < S > for String { fn encode (& self , s : & mut S) { s . emit_str (& self) ; } }
/* FP:serialize.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0028
/* FP:serialize.rs-0056 */ impl < D : Decoder > Decodable < D > for String { fn decode (d : & mut D) -> String { d . read_str () . to_owned () } }
/* FP:serialize.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0029
/* FP:serialize.rs-0058 */ impl < S : Encoder > Encodable < S > for () { fn encode (& self , _s : & mut S) { } }
/* FP:serialize.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0030
/* FP:serialize.rs-0060 */ impl < D : Decoder > Decodable < D > for () { fn decode (_ : & mut D) { } }
/* FP:serialize.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0031
/* FP:serialize.rs-0062 */ impl < S : Encoder , T > Encodable < S > for PhantomData < T > { fn encode (& self , _s : & mut S) { } }
/* FP:serialize.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0032
/* FP:serialize.rs-0064 */ impl < D : Decoder , T > Decodable < D > for PhantomData < T > { fn decode (_ : & mut D) -> PhantomData < T > { PhantomData } }
/* FP:serialize.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0033
/* FP:serialize.rs-0066 */ impl < D : Decoder , T : Decodable < D > > Decodable < D > for Box < [T] > { fn decode (d : & mut D) -> Box < [T] > { let v : Vec < T > = Decodable :: decode (d) ; v . into_boxed_slice () } }
/* FP:serialize.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0034
/* FP:serialize.rs-0068 */ impl < S : Encoder , T : Encodable < S > > Encodable < S > for Rc < T > { fn encode (& self , s : & mut S) { (* * self) . encode (s) ; } }
/* FP:serialize.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0035
/* FP:serialize.rs-0070 */ impl < D : Decoder , T : Decodable < D > > Decodable < D > for Rc < T > { fn decode (d : & mut D) -> Rc < T > { Rc :: new (Decodable :: decode (d)) } }
/* FP:serialize.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0036
/* FP:serialize.rs-0072 */ impl < S : Encoder , T : Encodable < S > > Encodable < S > for [T] { default fn encode (& self , s : & mut S) { s . emit_usize (self . len ()) ; for e in self { e . encode (s) ; } } }
/* FP:serialize.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0037
/* FP:serialize.rs-0074 */ impl < S : Encoder , T : Encodable < S > > Encodable < S > for Vec < T > { fn encode (& self , s : & mut S) { self . as_slice () . encode (s) ; } }
/* FP:serialize.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0038
/* FP:serialize.rs-0076 */ impl < D : Decoder , T : Decodable < D > > Decodable < D > for Vec < T > { default fn decode (d : & mut D) -> Vec < T > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
/* FP:serialize.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0039
/* FP:serialize.rs-0078 */ impl < S : Encoder , T : Encodable < S > , const N : usize > Encodable < S > for [T ; N] { fn encode (& self , s : & mut S) { self . as_slice () . encode (s) ; } }
/* FP:serialize.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0040
/* FP:serialize.rs-0080 */ impl < D : Decoder , const N : usize > Decodable < D > for [u8 ; N] { fn decode (d : & mut D) -> [u8 ; N] { let len = d . read_usize () ; assert ! (len == N) ; let mut v = [0u8 ; N] ; for i in 0 .. len { v [i] = Decodable :: decode (d) ; } v } }
/* FP:serialize.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0041
/* FP:serialize.rs-0082 */ impl < S : Encoder , T : Encodable < S > > Encodable < S > for Cow < '_ , [T] > where [T] : ToOwned < Owned = Vec < T > > , { fn encode (& self , s : & mut S) { let slice : & [T] = self ; slice . encode (s) ; } }
/* FP:serialize.rs-0083 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0042
/* FP:serialize.rs-0084 */ impl < D : Decoder , T : Decodable < D > + ToOwned > Decodable < D > for Cow < 'static , [T] > where [T] : ToOwned < Owned = Vec < T > > , { fn decode (d : & mut D) -> Cow < 'static , [T] > { let v : Vec < T > = Decodable :: decode (d) ; Cow :: Owned (v) } }
/* FP:serialize.rs-0085 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0043
/* FP:serialize.rs-0086 */ impl < S : Encoder > Encodable < S > for Cow < '_ , str > { fn encode (& self , s : & mut S) { let val : & str = self ; val . encode (s) } }
/* FP:serialize.rs-0087 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0044
/* FP:serialize.rs-0088 */ impl < D : Decoder > Decodable < D > for Cow < '_ , str > { fn decode (d : & mut D) -> Cow < 'static , str > { let v : String = Decodable :: decode (d) ; Cow :: Owned (v) } }
/* FP:serialize.rs-0089 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0045
/* FP:serialize.rs-0090 */ impl < S : Encoder , T : Encodable < S > > Encodable < S > for Option < T > { fn encode (& self , s : & mut S) { match * self { None => s . emit_u8 (0) , Some (ref v) => { s . emit_u8 (1) ; v . encode (s) ; } } } }
/* FP:serialize.rs-0091 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0046
/* FP:serialize.rs-0092 */ impl < D : Decoder , T : Decodable < D > > Decodable < D > for Option < T > { fn decode (d : & mut D) -> Option < T > { match d . read_u8 () { 0 => None , 1 => Some (Decodable :: decode (d)) , _ => panic ! ("Encountered invalid discriminant while decoding `Option`.") , } } }
/* FP:serialize.rs-0093 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0047
/* FP:serialize.rs-0094 */ impl < S : Encoder , T1 : Encodable < S > , T2 : Encodable < S > > Encodable < S > for Result < T1 , T2 > { fn encode (& self , s : & mut S) { match * self { Ok (ref v) => { s . emit_u8 (0) ; v . encode (s) ; } Err (ref v) => { s . emit_u8 (1) ; v . encode (s) ; } } } }
/* FP:serialize.rs-0095 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0048
/* FP:serialize.rs-0096 */ impl < D : Decoder , T1 : Decodable < D > , T2 : Decodable < D > > Decodable < D > for Result < T1 , T2 > { fn decode (d : & mut D) -> Result < T1 , T2 > { match d . read_u8 () { 0 => Ok (T1 :: decode (d)) , 1 => Err (T2 :: decode (d)) , _ => panic ! ("Encountered invalid discriminant while decoding `Result`.") , } } }
/* FP:serialize.rs-0097 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_MACRO_0049
/* FP:serialize.rs-0098 */ macro_rules ! peel { ($ name : ident , $ ($ other : ident ,) *) => (tuple ! { $ ($ other ,) * }) }
/* FP:serialize.rs-0099 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_MACRO_0050
/* FP:serialize.rs-0100 */ macro_rules ! tuple { () => () ; ($ ($ name : ident ,) +) => (impl < D : Decoder , $ ($ name : Decodable < D >) ,+> Decodable < D > for ($ ($ name ,) +) { fn decode (d : & mut D) -> ($ ($ name ,) +) { ($ ({ let element : $ name = Decodable :: decode (d) ; element } ,) +) } } impl < S : Encoder , $ ($ name : Encodable < S >) ,+> Encodable < S > for ($ ($ name ,) +) { # [allow (non_snake_case)] fn encode (& self , s : & mut S) { let ($ (ref $ name ,) +) = * self ; $ ($ name . encode (s) ;) + } } peel ! { $ ($ name ,) + }) }
/* FP:serialize.rs-0101 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_MACRO_0051
/* FP:serialize.rs-0102 */ tuple ! { T0 , T1 , T2 , T3 , T4 , T5 , T6 , T7 , T8 , T9 , T10 , T11 , }
/* FP:serialize.rs-0103 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0052
/* FP:serialize.rs-0104 */ impl < S : Encoder > Encodable < S > for path :: Path { fn encode (& self , e : & mut S) { self . to_str () . unwrap () . encode (e) ; } }
/* FP:serialize.rs-0105 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0053
/* FP:serialize.rs-0106 */ impl < S : Encoder > Encodable < S > for path :: PathBuf { fn encode (& self , e : & mut S) { path :: Path :: encode (self , e) ; } }
/* FP:serialize.rs-0107 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0054
/* FP:serialize.rs-0108 */ impl < D : Decoder > Decodable < D > for path :: PathBuf { fn decode (d : & mut D) -> path :: PathBuf { let bytes : String = Decodable :: decode (d) ; path :: PathBuf :: from (bytes) } }
/* FP:serialize.rs-0109 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0055
/* FP:serialize.rs-0110 */ impl < S : Encoder , T : Encodable < S > + Copy > Encodable < S > for Cell < T > { fn encode (& self , s : & mut S) { self . get () . encode (s) ; } }
/* FP:serialize.rs-0111 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0056
/* FP:serialize.rs-0112 */ impl < D : Decoder , T : Decodable < D > + Copy > Decodable < D > for Cell < T > { fn decode (d : & mut D) -> Cell < T > { Cell :: new (Decodable :: decode (d)) } }
/* FP:serialize.rs-0113 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0057
/* FP:serialize.rs-0114 */ impl < S : Encoder , T : Encodable < S > > Encodable < S > for RefCell < T > { fn encode (& self , s : & mut S) { self . borrow () . encode (s) ; } }
/* FP:serialize.rs-0115 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0058
/* FP:serialize.rs-0116 */ impl < D : Decoder , T : Decodable < D > > Decodable < D > for RefCell < T > { fn decode (d : & mut D) -> RefCell < T > { RefCell :: new (Decodable :: decode (d)) } }
/* FP:serialize.rs-0117 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0059
/* FP:serialize.rs-0118 */ impl < S : Encoder , T : Encodable < S > > Encodable < S > for Arc < T > { fn encode (& self , s : & mut S) { (* * self) . encode (s) ; } }
/* FP:serialize.rs-0119 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0060
/* FP:serialize.rs-0120 */ impl < D : Decoder , T : Decodable < D > > Decodable < D > for Arc < T > { fn decode (d : & mut D) -> Arc < T > { Arc :: new (Decodable :: decode (d)) } }
/* FP:serialize.rs-0121 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0061
/* FP:serialize.rs-0122 */ impl < S : Encoder , T : ? Sized + Encodable < S > > Encodable < S > for Box < T > { fn encode (& self , s : & mut S) { (* * self) . encode (s) } }
/* FP:serialize.rs-0123 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0062
/* FP:serialize.rs-0124 */ impl < D : Decoder , T : Decodable < D > > Decodable < D > for Box < T > { fn decode (d : & mut D) -> Box < T > { Box :: new (Decodable :: decode (d)) } }
/* FP:serialize.rs-0125 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0063
/* FP:serialize.rs-0126 */ impl < S : Encoder , A : Array < Item : Encodable < S > > > Encodable < S > for SmallVec < A > { fn encode (& self , s : & mut S) { self . as_slice () . encode (s) ; } }
/* FP:serialize.rs-0127 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0064
/* FP:serialize.rs-0128 */ impl < D : Decoder , A : Array < Item : Decodable < D > > > Decodable < D > for SmallVec < A > { fn decode (d : & mut D) -> SmallVec < A > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
/* FP:serialize.rs-0129 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0065
/* FP:serialize.rs-0130 */ impl < S : Encoder , T : Encodable < S > > Encodable < S > for ThinVec < T > { fn encode (& self , s : & mut S) { self . as_slice () . encode (s) ; } }
/* FP:serialize.rs-0131 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0066
/* FP:serialize.rs-0132 */ impl < D : Decoder , T : Decodable < D > > Decodable < D > for ThinVec < T > { fn decode (d : & mut D) -> ThinVec < T > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
/* FP:serialize.rs-0133 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0067
/* FP:serialize.rs-0134 */ impl < S : Encoder , T : Encodable < S > > Encodable < S > for VecDeque < T > { fn encode (& self , s : & mut S) { s . emit_usize (self . len ()) ; for e in self { e . encode (s) ; } } }
/* FP:serialize.rs-0135 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0068
/* FP:serialize.rs-0136 */ impl < D : Decoder , T : Decodable < D > > Decodable < D > for VecDeque < T > { fn decode (d : & mut D) -> VecDeque < T > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
/* FP:serialize.rs-0137 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0069
/* FP:serialize.rs-0138 */ impl < S : Encoder , K , V > Encodable < S > for BTreeMap < K , V > where K : Encodable < S > + PartialEq + Ord , V : Encodable < S > , { fn encode (& self , e : & mut S) { e . emit_usize (self . len ()) ; for (key , val) in self { key . encode (e) ; val . encode (e) ; } } }
/* FP:serialize.rs-0139 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0070
/* FP:serialize.rs-0140 */ impl < D : Decoder , K , V > Decodable < D > for BTreeMap < K , V > where K : Decodable < D > + PartialEq + Ord , V : Decodable < D > , { fn decode (d : & mut D) -> BTreeMap < K , V > { let len = d . read_usize () ; (0 .. len) . map (| _ | (Decodable :: decode (d) , Decodable :: decode (d))) . collect () } }
/* FP:serialize.rs-0141 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0071
/* FP:serialize.rs-0142 */ impl < S : Encoder , T > Encodable < S > for BTreeSet < T > where T : Encodable < S > + PartialEq + Ord , { fn encode (& self , s : & mut S) { s . emit_usize (self . len ()) ; for e in self { e . encode (s) ; } } }
/* FP:serialize.rs-0143 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0072
/* FP:serialize.rs-0144 */ impl < D : Decoder , T > Decodable < D > for BTreeSet < T > where T : Decodable < D > + PartialEq + Ord , { fn decode (d : & mut D) -> BTreeSet < T > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
/* FP:serialize.rs-0145 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0073
/* FP:serialize.rs-0146 */ impl < E : Encoder , K , V , S > Encodable < E > for HashMap < K , V , S > where K : Encodable < E > + Eq , V : Encodable < E > , S : BuildHasher , { fn encode (& self , e : & mut E) { e . emit_usize (self . len ()) ; for (key , val) in self { key . encode (e) ; val . encode (e) ; } } }
/* FP:serialize.rs-0147 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0074
/* FP:serialize.rs-0148 */ impl < D : Decoder , K , V , S > Decodable < D > for HashMap < K , V , S > where K : Decodable < D > + Hash + Eq , V : Decodable < D > , S : BuildHasher + Default , { fn decode (d : & mut D) -> HashMap < K , V , S > { let len = d . read_usize () ; (0 .. len) . map (| _ | (Decodable :: decode (d) , Decodable :: decode (d))) . collect () } }
/* FP:serialize.rs-0149 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0075
/* FP:serialize.rs-0150 */ impl < E : Encoder , T , S > Encodable < E > for HashSet < T , S > where T : Encodable < E > + Eq , S : BuildHasher , { fn encode (& self , s : & mut E) { s . emit_usize (self . len ()) ; for e in self { e . encode (s) ; } } }
/* FP:serialize.rs-0151 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0076
/* FP:serialize.rs-0152 */ impl < D : Decoder , T , S > Decodable < D > for HashSet < T , S > where T : Decodable < D > + Hash + Eq , S : BuildHasher + Default , { fn decode (d : & mut D) -> HashSet < T , S > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
/* FP:serialize.rs-0153 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0077
/* FP:serialize.rs-0154 */ impl < E : Encoder , K , V , S > Encodable < E > for indexmap :: IndexMap < K , V , S > where K : Encodable < E > + Hash + Eq , V : Encodable < E > , S : BuildHasher , { fn encode (& self , e : & mut E) { e . emit_usize (self . len ()) ; for (key , val) in self { key . encode (e) ; val . encode (e) ; } } }
/* FP:serialize.rs-0155 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0078
/* FP:serialize.rs-0156 */ impl < D : Decoder , K , V , S > Decodable < D > for indexmap :: IndexMap < K , V , S > where K : Decodable < D > + Hash + Eq , V : Decodable < D > , S : BuildHasher + Default , { fn decode (d : & mut D) -> indexmap :: IndexMap < K , V , S > { let len = d . read_usize () ; (0 .. len) . map (| _ | (Decodable :: decode (d) , Decodable :: decode (d))) . collect () } }
/* FP:serialize.rs-0157 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0079
/* FP:serialize.rs-0158 */ impl < E : Encoder , T , S > Encodable < E > for indexmap :: IndexSet < T , S > where T : Encodable < E > + Hash + Eq , S : BuildHasher , { fn encode (& self , s : & mut E) { s . emit_usize (self . len ()) ; for e in self { e . encode (s) ; } } }
/* FP:serialize.rs-0159 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0080
/* FP:serialize.rs-0160 */ impl < D : Decoder , T , S > Decodable < D > for indexmap :: IndexSet < T , S > where T : Decodable < D > + Hash + Eq , S : BuildHasher + Default , { fn decode (d : & mut D) -> indexmap :: IndexSet < T , S > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
/* FP:serialize.rs-0161 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0081
/* FP:serialize.rs-0162 */ impl < E : Encoder , T : Encodable < E > > Encodable < E > for Rc < [T] > { fn encode (& self , s : & mut E) { let slice : & [T] = self ; slice . encode (s) ; } }
/* FP:serialize.rs-0163 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0082
/* FP:serialize.rs-0164 */ impl < D : Decoder , T : Decodable < D > > Decodable < D > for Rc < [T] > { fn decode (d : & mut D) -> Rc < [T] > { let vec : Vec < T > = Decodable :: decode (d) ; vec . into () } }
/* FP:serialize.rs-0165 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0083
/* FP:serialize.rs-0166 */ impl < E : Encoder , T : Encodable < E > > Encodable < E > for Arc < [T] > { fn encode (& self , s : & mut E) { let slice : & [T] = self ; slice . encode (s) ; } }
/* FP:serialize.rs-0167 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0084
/* FP:serialize.rs-0168 */ impl < D : Decoder , T : Decodable < D > > Decodable < D > for Arc < [T] > { fn decode (d : & mut D) -> Arc < [T] > { let vec : Vec < T > = Decodable :: decode (d) ; vec . into () } }
/* FP:serialize.rs-0169 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0085
/* FP:serialize.rs-0170 */ impl < S : Encoder > Encodable < S > for Hash64 { # [inline] fn encode (& self , s : & mut S) { s . emit_raw_bytes (& self . as_u64 () . to_le_bytes ()) ; } }
/* FP:serialize.rs-0171 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0086
/* FP:serialize.rs-0172 */ impl < S : Encoder > Encodable < S > for Hash128 { # [inline] fn encode (& self , s : & mut S) { s . emit_raw_bytes (& self . as_u128 () . to_le_bytes ()) ; } }
/* FP:serialize.rs-0173 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0087
/* FP:serialize.rs-0174 */ impl < D : Decoder > Decodable < D > for Hash64 { # [inline] fn decode (d : & mut D) -> Self { Self :: new (u64 :: from_le_bytes (d . read_raw_bytes (8) . try_into () . unwrap ())) } }
/* FP:serialize.rs-0175 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_serialize_src_serialize_IMPL_0088
/* FP:serialize.rs-0176 */ impl < D : Decoder > Decodable < D > for Hash128 { # [inline] fn decode (d : & mut D) -> Self { Self :: new (u128 :: from_le_bytes (d . read_raw_bytes (16) . try_into () . unwrap ())) } }