macro_rules! deps {
    () => {
        StableHasher!();
        ExtendedHasher!();
        FromStableHash!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < H : ExtendedHasher > StableHasher < H > { # [doc = " Creates a new [`StableHasher`] from an already created [`ExtendedHasher`]."] # [doc = ""] # [doc = " Useful when wanting to initialize a hasher with different parameters/keys."] # [doc = ""] # [doc = " **Important**: Any use of the hasher before being given to a [`StableHasher`]"] # [doc = " is not covered by this crate guarentees and will make the resulting hash"] # [doc = " NOT platform independent."] # [inline] pub fn with_hasher (state : H) -> Self { StableHasher { state } } # [doc = " Returns the typed-hash value for the values written."] # [doc = ""] # [doc = " The resulting typed-hash value is constructed from an"] # [doc = " [`FromStableHash`] implemenation."] # [doc = ""] # [doc = " To be used in-place of [`Hasher::finish`]."] # [inline] # [must_use] pub fn finish < W : FromStableHash < Hash = H :: Hash > > (self) -> W { W :: from (self . state . finish ()) } }
    };
}

impl_32!()