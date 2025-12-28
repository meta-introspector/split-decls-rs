macro_rules! deps {
    () => {
        HashIgnoredAttrId!();
    };
}

macro_rules! HashStableContext {
    () => {
        deps!();
        # [doc = " Requirements for a `StableHashingContext` to be used in this crate."] # [doc = " This is a hack to allow using the `HashStable_Generic` derive macro"] # [doc = " instead of implementing everything in `rustc_middle`."] pub trait HashStableContext : rustc_ast :: HashStableContext + rustc_abi :: HashStableContext { fn hash_attr_id (& mut self , id : & HashIgnoredAttrId , hasher : & mut StableHasher) ; }
    };
}

HashStableContext!()