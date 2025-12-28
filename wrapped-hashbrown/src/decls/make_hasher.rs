macro_rules! make_hasher {
    () => {
        # [doc = " Ensures that a single closure type across uses of this which, in turn prevents multiple"] # [doc = " instances of any functions like `RawTable::reserve` from being generated"] # [cfg_attr (feature = "inline-more" , inline)] pub (crate) fn make_hasher < Q , V , S > (hash_builder : & S) -> impl Fn (& (Q , V)) -> u64 + '_ where Q : Hash , S : BuildHasher , { move | val | make_hash :: < Q , S > (hash_builder , & val . 0) }
    };
}

make_hasher!()