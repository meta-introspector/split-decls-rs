macro_rules! deps {
    () => {
        HeuristicFrequencyRank!();
        DefaultFrequencyRank!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl HeuristicFrequencyRank for DefaultFrequencyRank { fn rank (& self , byte : u8) -> u8 { self :: default_rank :: RANK [usize :: from (byte)] } }
    };
}

impl_85!();