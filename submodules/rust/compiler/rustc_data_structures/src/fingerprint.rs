mkuse!{use std :: hash :: { Hash , Hasher } ;}
mkuse!{use rustc_hashes :: Hash64 ;}
mkuse!{use rustc_serialize :: { Decodable , Decoder , Encodable , Encoder } ;}
mkuse!{use crate :: stable_hasher :: { FromStableHash , StableHasherHash , impl_stable_traits_for_trivial_type } ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkitem!{mkstruct!{# [derive (Eq , PartialEq , Ord , PartialOrd , Debug , Clone , Copy)] # [repr (C)] pub struct Fingerprint (u64 , u64) ;}}
mkitem!{mktrait!{pub trait FingerprintComponent { fn as_u64 (& self) -> u64 ; }}}
mkitem!{mkimpl!{impl FingerprintComponent for Hash64 { # [inline] fn as_u64 (& self) -> u64 { Hash64 :: as_u64 (* self) } }}}
mkitem!{mkimpl!{impl FingerprintComponent for u64 { # [inline] fn as_u64 (& self) -> u64 { * self } }}}
mkitem!{mkimpl!{impl Fingerprint { pub const ZERO : Fingerprint = Fingerprint (0 , 0) ; # [inline] pub fn new < A , B > (_0 : A , _1 : B) -> Fingerprint where A : FingerprintComponent , B : FingerprintComponent , { Fingerprint (_0 . as_u64 () , _1 . as_u64 ()) } # [inline] pub fn to_smaller_hash (& self) -> Hash64 { Hash64 :: new (self . 0 . wrapping_mul (3) . wrapping_add (self . 1)) } # [inline] pub fn split (& self) -> (Hash64 , Hash64) { (Hash64 :: new (self . 0) , Hash64 :: new (self . 1)) } # [inline] pub fn combine (self , other : Fingerprint) -> Fingerprint { Fingerprint (self . 0 . wrapping_mul (3) . wrapping_add (other . 0) , self . 1 . wrapping_mul (3) . wrapping_add (other . 1) ,) } # [inline] pub (crate) fn as_u128 (self) -> u128 { u128 :: from (self . 1) << 64 | u128 :: from (self . 0) } # [inline] pub fn combine_commutative (self , other : Fingerprint) -> Fingerprint { let a = u128 :: from (self . 1) << 64 | u128 :: from (self . 0) ; let b = u128 :: from (other . 1) << 64 | u128 :: from (other . 0) ; let c = a . wrapping_add (b) ; Fingerprint (c as u64 , (c >> 64) as u64) } pub fn to_hex (& self) -> String { format ! ("{:x}{:x}" , self . 0 , self . 1) } # [inline] pub fn to_le_bytes (& self) -> [u8 ; 16] { let mut result = [0u8 ; 16] ; let first_half : & mut [u8 ; 8] = (& mut result [0 .. 8]) . try_into () . unwrap () ; * first_half = self . 0 . to_le_bytes () ; let second_half : & mut [u8 ; 8] = (& mut result [8 .. 16]) . try_into () . unwrap () ; * second_half = self . 1 . to_le_bytes () ; result } # [inline] pub fn from_le_bytes (bytes : [u8 ; 16]) -> Fingerprint { Fingerprint (u64 :: from_le_bytes (bytes [0 .. 8] . try_into () . unwrap ()) , u64 :: from_le_bytes (bytes [8 .. 16] . try_into () . unwrap ()) ,) } }}}
mkitem!{mkimpl!{impl std :: fmt :: Display for Fingerprint { fn fmt (& self , formatter : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (formatter , "{:x}-{:x}" , self . 0 , self . 1) } }}}
mkitem!{mkimpl!{impl Hash for Fingerprint { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { state . write_fingerprint (self) ; } }}}
mkitem!{mktrait!{trait FingerprintHasher { fn write_fingerprint (& mut self , fingerprint : & Fingerprint) ; }}}
mkitem!{mkimpl!{impl < H : Hasher > FingerprintHasher for H { # [inline] default fn write_fingerprint (& mut self , fingerprint : & Fingerprint) { self . write_u64 (fingerprint . 0) ; self . write_u64 (fingerprint . 1) ; } }}}
mkitem!{mkimpl!{impl FingerprintHasher for crate :: unhash :: Unhasher { # [inline] fn write_fingerprint (& mut self , fingerprint : & Fingerprint) { self . write_u64 (fingerprint . 0 . wrapping_add (fingerprint . 1)) ; } }}}
mkitem!{mkimpl!{impl FromStableHash for Fingerprint { type Hash = StableHasherHash ; # [inline] fn from (StableHasherHash ([_0 , _1]) : Self :: Hash) -> Self { Fingerprint (_0 , _1) } }}}
mkitem!{impl_stable_traits_for_trivial_type ! (Fingerprint) ;}
mkitem!{mkimpl!{impl < E : Encoder > Encodable < E > for Fingerprint { # [inline] fn encode (& self , s : & mut E) { s . emit_raw_bytes (& self . to_le_bytes ()) ; } }}}
mkitem!{mkimpl!{impl < D : Decoder > Decodable < D > for Fingerprint { # [inline] fn decode (d : & mut D) -> Self { Fingerprint :: from_le_bytes (d . read_raw_bytes (16) . try_into () . unwrap ()) } }}}
mkitem!{mkstruct!{# [cfg_attr (any (target_arch = "x86" , target_arch = "x86_64") , repr (packed))] # [derive (Eq , PartialEq , Ord , PartialOrd , Debug , Clone , Copy , Hash)] pub struct PackedFingerprint (Fingerprint) ;}}
mkitem!{mkimpl!{impl std :: fmt :: Display for PackedFingerprint { # [inline] fn fmt (& self , formatter : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let copy = self . 0 ; copy . fmt (formatter) } }}}
mkitem!{mkimpl!{impl < E : Encoder > Encodable < E > for PackedFingerprint { # [inline] fn encode (& self , s : & mut E) { let copy = self . 0 ; copy . encode (s) ; } }}}
mkitem!{mkimpl!{impl < D : Decoder > Decodable < D > for PackedFingerprint { # [inline] fn decode (d : & mut D) -> Self { Self (Fingerprint :: decode (d)) } }}}
mkitem!{mkimpl!{impl From < Fingerprint > for PackedFingerprint { # [inline] fn from (f : Fingerprint) -> PackedFingerprint { PackedFingerprint (f) } }}}
mkitem!{mkimpl!{impl From < PackedFingerprint > for Fingerprint { # [inline] fn from (f : PackedFingerprint) -> Fingerprint { f . 0 } }}}