macro_rules! DefaultHasher {
    () => {
        # [doc = " Default hasher for [`HashMap`](crate::HashMap) and [`HashSet`](crate::HashSet)."] # [cfg (feature = "default-hasher")] # [derive (Clone)] pub struct DefaultHasher { inner : < RandomState as BuildHasher > :: Hasher , }
    };
}

DefaultHasher!();