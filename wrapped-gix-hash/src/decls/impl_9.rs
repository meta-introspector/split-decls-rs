macro_rules! deps {
    () => {
        ObjectId!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        # [allow (clippy :: derived_hash_with_manual_eq)] impl Hash for ObjectId { fn hash < H : Hasher > (& self , state : & mut H) { state . write (self . as_slice ()) ; } }
    };
}

impl_9!();