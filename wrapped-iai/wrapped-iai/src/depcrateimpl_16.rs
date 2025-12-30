// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl CachegrindStats { pub fn ram_accesses (& self) -> u64 { self . instruction_cache_misses + self . data_cache_read_misses + self . data_cache_write_misses } pub fn summarize (& self) -> CachegrindSummary { let ram_hits = self . ram_accesses () ; let l3_accesses = self . instruction_l1_misses + self . data_l1_read_misses + self . data_l1_write_misses ; let l3_hits = l3_accesses - ram_hits ; let total_memory_rw = self . instruction_reads + self . data_reads + self . data_writes ; let l1_hits = total_memory_rw - (ram_hits + l3_hits) ; CachegrindSummary { l1_hits , l3_hits , ram_hits , } } # [rustfmt :: skip] pub fn subtract (& self , calibration : & CachegrindStats) -> CachegrindStats { CachegrindStats { instruction_reads : self . instruction_reads . saturating_sub (calibration . instruction_reads) , instruction_l1_misses : self . instruction_l1_misses . saturating_sub (calibration . instruction_l1_misses) , instruction_cache_misses : self . instruction_cache_misses . saturating_sub (calibration . instruction_cache_misses) , data_reads : self . data_reads . saturating_sub (calibration . data_reads) , data_l1_read_misses : self . data_l1_read_misses . saturating_sub (calibration . data_l1_read_misses) , data_cache_read_misses : self . data_cache_read_misses . saturating_sub (calibration . data_cache_read_misses) , data_writes : self . data_writes . saturating_sub (calibration . data_writes) , data_l1_write_misses : self . data_l1_write_misses . saturating_sub (calibration . data_l1_write_misses) , data_cache_write_misses : self . data_cache_write_misses . saturating_sub (calibration . data_cache_write_misses) , } } }
};
}
