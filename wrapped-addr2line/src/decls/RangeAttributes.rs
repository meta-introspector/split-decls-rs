macro_rules! RangeAttributes {
    () => {
        struct RangeAttributes < R : gimli :: Reader > { low_pc : Option < u64 > , high_pc : Option < u64 > , size : Option < u64 > , ranges_offset : Option < gimli :: RangeListsOffset < < R as gimli :: Reader > :: Offset > > , }
    };
}

RangeAttributes!();