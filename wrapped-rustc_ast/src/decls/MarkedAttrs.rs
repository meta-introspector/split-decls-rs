macro_rules! MarkedAttrs {
    () => {
        pub struct MarkedAttrs (GrowableBitSet < AttrId >) ;
    };
}

MarkedAttrs!()