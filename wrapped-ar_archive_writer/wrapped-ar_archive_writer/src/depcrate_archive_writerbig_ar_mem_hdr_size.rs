// Generated macro for BIG_AR_MEM_HDR_SIZE (const)
macro_rules! Depcrate_archive_writerBIG_AR_MEM_HDR_SIZE {
() => {
// Module: crate::archive_writer
// Provides: {"BIG_AR_MEM_HDR_SIZE"}
// Dependencies: {}
const BIG_AR_MEM_HDR_SIZE : u64 = { assert ! (std :: mem :: size_of ::< usize > () <= std :: mem :: size_of ::< u64 > () || std :: mem :: size_of ::< big_archive :: BigArMemHdrType > () < u64 :: MAX as usize) ; std :: mem :: size_of :: < big_archive :: BigArMemHdrType > () as u64 } ;
};
}
