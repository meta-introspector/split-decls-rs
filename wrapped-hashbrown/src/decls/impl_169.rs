macro_rules! deps {
    () => {
        ParDrainProducer!();
        RawIntoParIter!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < T : Send , A : Allocator + Send > ParallelIterator for RawIntoParIter < T , A > { type Item = T ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let iter = unsafe { self . table . iter () . iter } ; let _guard = guard (self . table . into_allocation () , | alloc | { if let Some ((ptr , layout , ref alloc)) = * alloc { unsafe { alloc . deallocate (ptr , layout) ; } } }) ; let producer = ParDrainProducer { iter } ; plumbing :: bridge_unindexed (producer , consumer) } }
    };
}

impl_169!()