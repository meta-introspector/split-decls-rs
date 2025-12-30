// Generated macro for MIN_PAGE_SIZE (const)
macro_rules! Depcrate_serializationMIN_PAGE_SIZE {
() => {
// Module: crate::serialization
// Provides: {"MIN_PAGE_SIZE"}
// Dependencies: {}
# [doc = " The number of bytes we consider enough to warrant their own page when"] # [doc = " deciding whether to flush a partially full buffer. Actual pages may need"] # [doc = " to be smaller, e.g. when writing the tail of the data stream."] const MIN_PAGE_SIZE : usize = MAX_PAGE_SIZE / 2 ;
};
}
