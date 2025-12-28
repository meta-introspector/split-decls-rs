macro_rules! RefStore {
    () => {
        # [doc = " The standard type for a store to handle git references."] pub type RefStore = gix_ref :: file :: Store ;
    };
}

RefStore!()