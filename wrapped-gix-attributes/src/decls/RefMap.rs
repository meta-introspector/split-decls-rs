macro_rules! deps {
    () => {
        RefMapKey!();
    };
}

macro_rules! RefMap {
    () => {
        deps!();
        # [derive (Clone)] pub (crate) struct RefMap < T > (BTreeMap < RefMapKey , T >) ;
    };
}

RefMap!()