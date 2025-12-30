// Generated macro for impl_98 (impl)
macro_rules! Depcrate_arch_all_packedpairimpl_98 {
() => {
// Module: crate::arch::all::packedpair
// Provides: {"impl_98"}
// Dependencies: {}
# [doc = " This permits passing any implementation of `HeuristicFrequencyRank` as a"] # [doc = " borrowed version of itself."] impl < 'a , R > HeuristicFrequencyRank for & 'a R where R : HeuristicFrequencyRank , { fn rank (& self , byte : u8) -> u8 { (* * self) . rank (byte) } }
};
}
