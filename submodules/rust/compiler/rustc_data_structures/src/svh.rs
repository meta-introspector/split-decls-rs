mkuse!{use std :: fmt ;}
mkuse!{use rustc_macros :: { Decodable_NoContext , Encodable_NoContext } ;}
mkuse!{use crate :: fingerprint :: Fingerprint ;}
mkuse!{use crate :: stable_hasher ;}
mkitem!{mkstruct!{# [derive (Copy , Clone , PartialEq , Eq , Debug , Encodable_NoContext , Decodable_NoContext , Hash)] pub struct Svh { hash : Fingerprint , }}}
mkitem!{mkimpl!{impl Svh { # [doc = " Creates a new `Svh` given the hash. If you actually want to"] # [doc = " compute the SVH from some HIR, you want the `calculate_svh`"] # [doc = " function found in `rustc_incremental`."] pub fn new (hash : Fingerprint) -> Svh { Svh { hash } } pub fn as_u128 (self) -> u128 { self . hash . as_u128 () } pub fn to_hex (self) -> String { format ! ("{:032x}" , self . hash . as_u128 ()) } }}}
mkitem!{mkimpl!{impl fmt :: Display for Svh { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (& self . to_hex ()) } }}}
mkitem!{mkimpl!{impl < T > stable_hasher :: HashStable < T > for Svh { # [inline] fn hash_stable (& self , ctx : & mut T , hasher : & mut stable_hasher :: StableHasher) { let Svh { hash } = * self ; hash . hash_stable (ctx , hasher) ; } }}}