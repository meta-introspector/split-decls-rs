macro_rules! AttrIdGenerator {
    () => {
        pub struct AttrIdGenerator (AtomicU32) ;
    };
}

AttrIdGenerator!();