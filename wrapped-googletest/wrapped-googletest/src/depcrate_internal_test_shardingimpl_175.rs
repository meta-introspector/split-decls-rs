// Generated macro for impl_175 (impl)
macro_rules! Depcrate_internal_test_shardingimpl_175 {
() => {
// Module: crate::internal::test_sharding
// Provides: {"impl_175"}
// Dependencies: {}
impl Sharding { fn test_should_run (& self , test_case_hash : u64) -> bool { (test_case_hash % self . total_shards . get ()) == self . this_shard } fn from_environment () -> Sharding { let this_shard : Option < u64 > = { get_var (TEST_SHARD_INDEX) . and_then (| value | value . parse () . ok ()) } ; let total_shards : Option < NonZeroU64 > = { get_var (TEST_TOTAL_SHARDS) . and_then (| value | value . parse () . ok ()) . and_then (NonZeroU64 :: new) } ; match (this_shard , total_shards) { (Some (this_shard) , Some (total_shards)) if this_shard < total_shards . get () => { if let Some (name) = get_var_os (TEST_SHARD_STATUS_FILE) { let pathbuf = PathBuf :: from (name) ; if let Err (e) = create_status_file (& pathbuf) { eprintln ! ("failed to create $GTEST_SHARD_STATUS_FILE file {}: {}" , pathbuf . display () , e) ; } } Sharding { this_shard , total_shards } } _ => Sharding :: default () , } } }
};
}
