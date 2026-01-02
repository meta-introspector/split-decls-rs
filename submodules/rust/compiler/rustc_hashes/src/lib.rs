mkuse!{use std :: fmt ;}
mkuse!{use std :: ops :: BitXorAssign ;}
mkuse!{use rustc_stable_hash :: { FromStableHash , SipHasher128Hash as StableHasherHash } ;}
mkitem!{mkstruct!{# [doc = " A `u64` but encoded with a fixed size; for hashes this encoding is more compact than `u64`."] # [derive (Clone , Copy , PartialEq , Eq , Hash , PartialOrd , Ord , Default)] pub struct Hash64 { inner : u64 , }}}
mkitem!{mkimpl!{impl Hash64 { pub const ZERO : Hash64 = Hash64 { inner : 0 } ; # [inline] pub fn new (n : u64) -> Self { Self { inner : n } } # [inline] pub fn as_u64 (self) -> u64 { self . inner } # [inline] pub fn wrapping_add (self , other : Self) -> Self { Self { inner : self . inner . wrapping_add (other . inner) } } }}}
mkitem!{mkimpl!{impl BitXorAssign < u64 > for Hash64 { # [inline] fn bitxor_assign (& mut self , rhs : u64) { self . inner ^= rhs ; } }}}
mkitem!{mkimpl!{impl FromStableHash for Hash64 { type Hash = StableHasherHash ; # [inline] fn from (StableHasherHash ([_0 , __1]) : Self :: Hash) -> Self { Self { inner : _0 } } }}}
mkitem!{mkimpl!{impl fmt :: Debug for Hash64 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . inner . fmt (f) } }}}
mkitem!{mkimpl!{impl fmt :: LowerHex for Hash64 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (& self . inner , f) } }}}
mkitem!{mkstruct!{# [doc = " A `u128` but encoded with a fixed size; for hashes this encoding is more compact than `u128`."] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Default)] pub struct Hash128 { inner : u128 , }}}
mkitem!{mkimpl!{impl std :: hash :: Hash for Hash128 { fn hash < H : std :: hash :: Hasher > (& self , h : & mut H) { h . write_u64 (self . truncate () . as_u64 ()) ; } }}}
mkitem!{mkimpl!{impl Hash128 { # [inline] pub fn new (n : u128) -> Self { Self { inner : n } } # [inline] pub fn truncate (self) -> Hash64 { Hash64 { inner : self . inner as u64 } } # [inline] pub fn wrapping_add (self , other : Self) -> Self { Self { inner : self . inner . wrapping_add (other . inner) } } # [inline] pub fn as_u128 (self) -> u128 { self . inner } }}}
mkitem!{mkimpl!{impl FromStableHash for Hash128 { type Hash = StableHasherHash ; # [inline] fn from (StableHasherHash ([_0 , _1]) : Self :: Hash) -> Self { Self { inner : u128 :: from (_0) | (u128 :: from (_1) << 64) } } }}}
mkitem!{mkimpl!{impl fmt :: Debug for Hash128 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . inner . fmt (f) } }}}
mkitem!{mkimpl!{impl fmt :: LowerHex for Hash128 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (& self . inner , f) } }}}