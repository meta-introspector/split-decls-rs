// Generated macro for MemoryCellClocks (struct)
macro_rules! Depcrate_concurrency_data_raceMemoryCellClocks {
() => {
// Module: crate::concurrency::data_race
// Provides: {"MemoryCellClocks"}
// Dependencies: {}
# [doc = " Per-byte vector clock metadata for data-race detection."] # [derive (Clone , PartialEq , Eq , Debug)] struct MemoryCellClocks { # [doc = " The vector clock timestamp and the thread that did the last non-atomic write. We don't need"] # [doc = " a full `VClock` here, it's always a single thread and nothing synchronizes, so the effective"] # [doc = " clock is all-0 except for the thread that did the write."] write : (VectorIdx , VTimestamp) , # [doc = " The type of operation that the write index represents,"] # [doc = " either newly allocated memory, a non-atomic write or"] # [doc = " a deallocation of memory."] write_type : NaWriteType , # [doc = " The vector clock of all non-atomic reads that happened since the last non-atomic write"] # [doc = " (i.e., we join together the \"singleton\" clocks corresponding to each read). It is reset to"] # [doc = " zero on each write operation."] read : VClock , # [doc = " Atomic access, acquire, release sequence tracking clocks."] # [doc = " For non-atomic memory this value is set to None."] # [doc = " For atomic memory, each byte carries this information."] atomic_ops : Option < Box < AtomicMemoryCellClocks > > , }
};
}
