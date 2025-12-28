macro_rules! deps {
    () => {
        AttrIdGenerator!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        impl AttrIdGenerator { pub fn new () -> Self { AttrIdGenerator (AtomicU32 :: new (0)) } pub fn mk_attr_id (& self) -> AttrId { let id = self . 0 . fetch_add (1 , Ordering :: Relaxed) ; assert ! (id != u32 :: MAX) ; AttrId :: from_u32 (id) } }
    };
}

impl_274!();