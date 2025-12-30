// Generated macro for GENMC_GLOBAL_ADDRESSES_MASK (const)
macro_rules! DepcrateGENMC_GLOBAL_ADDRESSES_MASK {
() => {
// Module: crate
// Provides: {"GENMC_GLOBAL_ADDRESSES_MASK"}
// Dependencies: {}
# [doc = " Defined in \"genmc/src/Support/SAddr.hpp\"."] # [doc = " The first bit of all global addresses must be set to `1`."] # [doc = " This means the mask, interpreted as an address, is the lower bound of where the global address space starts."] # [doc = ""] # [doc = " FIXME(genmc): rework this if non-64bit support is added to GenMC (the current allocation scheme only allows for 64bit addresses)."] # [doc = " FIXME(genmc): currently we use `get_global_alloc_static_mask()` to ensure the constant is consistent between Miri and GenMC,"] # [doc = "   but if https://github.com/dtolnay/cxx/issues/1051 is fixed we could share the constant directly."] pub const GENMC_GLOBAL_ADDRESSES_MASK : u64 = 1 << 63 ;
};
}
