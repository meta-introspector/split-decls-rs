macro_rules! deps {
    () => {
        RangeAttributes!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < R : gimli :: Reader > Default for RangeAttributes < R > { fn default () -> Self { RangeAttributes { low_pc : None , high_pc : None , size : None , ranges_offset : None , } } }
    };
}

impl_101!();