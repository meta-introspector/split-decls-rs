// Generated macro for _impl (module)
macro_rules! Depcrate_hasher_impl {
() => {
// Module: crate::hasher
// Provides: {"_impl"}
// Dependencies: {}
pub (super) mod _impl { use sha1_checked :: { CollisionResult , Digest } ; use crate :: hasher :: Error ; # [doc = " An implementation of the Sha1 hash, which can be used once."] # [doc = ""] # [doc = " We use [`sha1_checked`] to implement the same collision detection"] # [doc = " algorithm as Git."] # [derive (Clone)] pub struct Hasher (sha1_checked :: Sha1) ; impl Hasher { # [doc = " Let's not provide a public default implementation to force people to go through [`hasher()`]."] fn default () -> Self { Self (sha1_checked :: Builder :: default () . safe_hash (false) . build ()) } } impl Hasher { # [doc = " Digest the given `bytes`."] pub fn update (& mut self , bytes : & [u8]) { self . 0 . update (bytes) ; } # [doc = " Finalize the hash and produce an object ID."] # [doc = ""] # [doc = " Returns [`Error`] if a collision attack is detected."] # [inline] pub fn try_finalize (self) -> Result < crate :: ObjectId , Error > { match self . 0 . try_finalize () { CollisionResult :: Ok (digest) => Ok (crate :: ObjectId :: Sha1 (digest . into ())) , CollisionResult :: Mitigated (_) => { # [allow (unsafe_code)] unsafe { std :: hint :: unreachable_unchecked () } } CollisionResult :: Collision (digest) => Err (Error :: CollisionAttack { digest : crate :: ObjectId :: Sha1 (digest . into ()) , }) , } } } # [doc = " Produce a hasher suitable for the given `kind` of hash."] # [inline] pub fn hasher (kind : crate :: Kind) -> Hasher { match kind { crate :: Kind :: Sha1 => Hasher :: default () , } } }
};
}
