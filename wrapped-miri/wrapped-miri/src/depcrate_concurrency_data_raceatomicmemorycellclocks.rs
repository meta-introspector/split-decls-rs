// Generated macro for AtomicMemoryCellClocks (struct)
macro_rules! Depcrate_concurrency_data_raceAtomicMemoryCellClocks {
() => {
// Module: crate::concurrency::data_race
// Provides: {"AtomicMemoryCellClocks"}
// Dependencies: {}
# [doc = " Externally stored memory cell clocks"] # [doc = " explicitly to reduce memory usage for the"] # [doc = " common case where no atomic operations"] # [doc = " exists on the memory cell."] # [derive (Clone , PartialEq , Eq , Debug)] struct AtomicMemoryCellClocks { # [doc = " The clock-vector of the timestamp of the last atomic"] # [doc = " read operation performed by each thread."] # [doc = " This detects potential data-races between atomic read"] # [doc = " and non-atomic write operations."] read_vector : VClock , # [doc = " The clock-vector of the timestamp of the last atomic"] # [doc = " write operation performed by each thread."] # [doc = " This detects potential data-races between atomic write"] # [doc = " and non-atomic read or write operations."] write_vector : VClock , # [doc = " Synchronization vector for acquire-release semantics"] # [doc = " contains the vector of timestamps that will"] # [doc = " happen-before a thread if an acquire-load is"] # [doc = " performed on the data."] sync_vector : VClock , # [doc = " The size of accesses to this atomic location."] # [doc = " We use this to detect non-synchronized mixed-size accesses. Since all accesses must be"] # [doc = " aligned to their size, this is sufficient to detect imperfectly overlapping accesses."] # [doc = " `None` indicates that we saw multiple different sizes, which is okay as long as all accesses are reads."] size : Option < Size > , }
};
}
