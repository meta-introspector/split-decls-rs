macro_rules! deps {
    () => {
        HeuristicFrequencyRank!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        # [doc = " This permits passing any implementation of `HeuristicFrequencyRank` as a"] # [doc = " borrowed version of itself."] impl < 'a , R > HeuristicFrequencyRank for & 'a R where R : HeuristicFrequencyRank , { fn rank (& self , byte : u8) -> u8 { (* * self) . rank (byte) } }
    };
}

impl_86!()