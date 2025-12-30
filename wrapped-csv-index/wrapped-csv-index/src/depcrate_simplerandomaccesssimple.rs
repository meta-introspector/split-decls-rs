// Generated macro for RandomAccessSimple (struct)
macro_rules! Depcrate_simpleRandomAccessSimple {
() => {
// Module: crate::simple
// Provides: {"RandomAccessSimple"}
// Dependencies: {}
# [doc = " A simple index for random access to CSV records."] # [doc = ""] # [doc = " This index permits seeking to the start of any CSV record with a constant"] # [doc = " number of operations."] # [doc = ""] # [doc = " The format of the index is simplistic and amenable to serializing to disk."] # [doc = " It consists of exactly `N+1` 64 bit big-endian integers, where `N` is the"] # [doc = " number of records in the CSV data that is indexed. Each `i`th integer"] # [doc = " corresponds to the approximate byte offset where the `i`th record in the"] # [doc = " CSV data begins. One additional integer is written to the end of the index"] # [doc = " which indicates the total number of records in the CSV data."] # [doc = ""] # [doc = " This indexing format does not store the line numbers of CSV records, so"] # [doc = " using the positions returned by this index to seek a CSV reader will likely"] # [doc = " cause any future line numbers reported by that reader to be incorrect."] # [doc = ""] # [doc = " This format will never change."] # [doc = ""] # [doc = " N.B. The format of this indexing scheme matches the format of the old the"] # [doc = " `csv::Indexed` type in pre-1.0 versions of the `csv` crate."] pub struct RandomAccessSimple < R > { rdr : R , len : u64 , }
};
}
