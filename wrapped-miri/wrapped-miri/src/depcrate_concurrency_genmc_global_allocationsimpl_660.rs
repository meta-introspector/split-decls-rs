// Generated macro for impl_660 (impl)
macro_rules! Depcrate_concurrency_genmc_global_allocationsimpl_660 {
() => {
// Module: crate::concurrency::genmc::global_allocations
// Provides: {"impl_660"}
// Dependencies: {}
impl GlobalAllocationHandler { # [doc = " Create a new global address generator with a given max address `last_addr`"] # [doc = " (corresponding to the highest address available on the target platform, unless another limit exists)."] # [doc = " No addresses higher than this will be allocated."] # [doc = " Will panic if the given address limit is too small to allocate any addresses."] pub fn new (last_addr : u64) -> GlobalAllocationHandler { assert_eq ! (GENMC_GLOBAL_ADDRESSES_MASK , get_global_alloc_static_mask ()) ; assert_ne ! (GENMC_GLOBAL_ADDRESSES_MASK , 0) ; assert ! (GENMC_GLOBAL_ADDRESSES_MASK < last_addr , "only 64bit platforms are currently supported (highest address {last_addr:#x} <= minimum global address {GENMC_GLOBAL_ADDRESSES_MASK:#x}).") ; Self (RwLock :: new (GlobalStateInner { base_addr : FxHashMap :: default () , address_generator : AddressGenerator :: new (GENMC_GLOBAL_ADDRESSES_MASK .. last_addr) , rng : StdRng :: seed_from_u64 (0) , })) } }
};
}
