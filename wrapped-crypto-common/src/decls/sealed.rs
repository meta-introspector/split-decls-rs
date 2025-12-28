macro_rules! deps {
    () => {
        BlockSizes!();
        Output!();
    };
}

macro_rules! sealed {
    () => {
        deps!();
        mod sealed { use crate :: typenum :: { IsLess , NonZero , True , U256 , Unsigned } ; pub trait BlockSizes { } impl < T : Unsigned > BlockSizes for T where Self : IsLess < U256 , Output = True > + NonZero { } }
    };
}

sealed!()