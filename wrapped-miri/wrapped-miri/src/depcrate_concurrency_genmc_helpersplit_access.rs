// Generated macro for split_access (function)
macro_rules! Depcrate_concurrency_genmc_helpersplit_access {
() => {
// Module: crate::concurrency::genmc::helper
// Provides: {"split_access"}
// Dependencies: {}
# [doc = " This function is used to split up a large memory access into aligned, non-overlapping chunks of a limited size."] # [doc = " Returns an iterator over the chunks, yielding `(base address, size)` of each chunk, ordered by address."] pub fn split_access (address : Size , size : Size) -> impl Iterator < Item = (u64 , u64) > { let start_address = address . bytes () ; let end_address = start_address + size . bytes () ; let start_address_aligned = start_address . next_multiple_of (MAX_ACCESS_SIZE) ; let end_address_aligned = (end_address / MAX_ACCESS_SIZE) * MAX_ACCESS_SIZE ; debug ! ("GenMC: splitting NA memory access into {MAX_ACCESS_SIZE} byte chunks: {}B + {} * {MAX_ACCESS_SIZE}B + {}B = {size:?}" , start_address_aligned - start_address , (end_address_aligned - start_address_aligned) / MAX_ACCESS_SIZE , end_address - end_address_aligned ,) ; let start_chunks = (start_address .. start_address_aligned) . map (| address | (address , 1)) ; let aligned_chunks = (start_address_aligned .. end_address_aligned) . step_by (MAX_ACCESS_SIZE . try_into () . unwrap ()) . map (| address | (address , MAX_ACCESS_SIZE)) ; let end_chunks = (end_address_aligned .. end_address) . map (| address | (address , 1)) ; start_chunks . chain (aligned_chunks) . chain (end_chunks) }
};
}
