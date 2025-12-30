// Generated macro for PhfHash (trait)
macro_rules! DepcratePhfHash {
() => {
// Module: crate
// Provides: {"PhfHash"}
// Dependencies: {}
# [doc = " A trait implemented by types which can be used in PHF data structures."] # [doc = ""] # [doc = " This differs from the standard library's `Hash` trait in that `PhfHash`'s"] # [doc = " results must be architecture independent so that hashes will be consistent"] # [doc = " between the host and target when cross compiling."] pub trait PhfHash { # [doc = " Feeds the value into the state given, updating the hasher as necessary."] fn phf_hash < H : Hasher > (& self , state : & mut H) ; # [doc = " Feeds a slice of this type into the state provided."] fn phf_hash_slice < H : Hasher > (data : & [Self] , state : & mut H) where Self : Sized , { for piece in data { piece . phf_hash (state) ; } } }
};
}
