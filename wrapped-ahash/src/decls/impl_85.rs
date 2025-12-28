macro_rules! deps {
    () => {
        RandomState!();
        AHasherFixed!();
        AHasherStr!();
        AHasherU64!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        # [cfg (specialize)] impl RandomState { # [inline] pub (crate) fn hash_as_u64 < T : Hash + ? Sized > (& self , value : & T) -> u64 { let mut hasher = AHasherU64 { buffer : self . k1 , pad : self . k0 , } ; value . hash (& mut hasher) ; hasher . finish () } # [inline] pub (crate) fn hash_as_fixed_length < T : Hash + ? Sized > (& self , value : & T) -> u64 { let mut hasher = AHasherFixed (self . build_hasher ()) ; value . hash (& mut hasher) ; hasher . finish () } # [inline] pub (crate) fn hash_as_str < T : Hash + ? Sized > (& self , value : & T) -> u64 { let mut hasher = AHasherStr (self . build_hasher ()) ; value . hash (& mut hasher) ; hasher . finish () } }
    };
}

impl_85!()