// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl Default for GenesisConfig { fn default () -> Self { Self { creation_time : SystemTime :: now () . duration_since (UNIX_EPOCH) . unwrap () . as_secs () as UnixTimestamp , accounts : BTreeMap :: default () , native_instruction_processors : Vec :: default () , rewards_pools : BTreeMap :: default () , ticks_per_slot : DEFAULT_TICKS_PER_SLOT , unused : UNUSED_DEFAULT , poh_config : PohConfig :: default () , inflation : Inflation :: default () , __backwards_compat_with_v0_23 : 0 , fee_rate_governor : FeeRateGovernor :: default () , rent : Rent :: default () , epoch_schedule : EpochSchedule :: default () , cluster_type : ClusterType :: Development , } } }
};
}
