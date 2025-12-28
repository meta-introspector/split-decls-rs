macro_rules! deps {
    () => {
        RandomSource!();
    };
}

macro_rules! RandomState {
    () => {
        deps!();
        # [doc = " Provides a [Hasher] factory. This is typically used (e.g. by [HashMap]) to create"] # [doc = " [AHasher]s in order to hash the keys of the map. See `build_hasher` below."] # [doc = ""] # [doc = " [build_hasher]: ahash::"] # [doc = " [Hasher]: std::hash::Hasher"] # [doc = " [BuildHasher]: std::hash::BuildHasher"] # [doc = " [HashMap]: std::collections::HashMap"] # [doc = ""] # [doc = " There are multiple constructors each is documented in more detail below:"] # [doc = ""] # [doc = " | Constructor   | Dynamically random? | Seed |"] # [doc = " |---------------|---------------------|------|"] # [doc = " |`new`          | Each instance unique|_[RandomSource]_|"] # [doc = " |`generate_with`| Each instance unique|`u64` x 4 + [RandomSource]|"] # [doc = " |`with_seed`    | Fixed per process   |`u64` + static random number|"] # [doc = " |`with_seeds`   | Fixed               |`u64` x 4|"] # [doc = ""] # [derive (Clone)] pub struct RandomState { pub (crate) k0 : u64 , pub (crate) k1 : u64 , pub (crate) k2 : u64 , pub (crate) k3 : u64 , }
    };
}

RandomState!();