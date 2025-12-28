macro_rules! RefCount {
    () => {
        # [repr (transparent)] # [derive (Default)] pub struct RefCount (pub (crate) AtomicI32) ;
    };
}

RefCount!()