// Generated macro for BinIndex (struct)
macro_rules! Depcrate_binaryBinIndex {
() => {
// Module: crate::binary
// Provides: {"BinIndex"}
// Dependencies: {}
# [doc = " The `BinIndex` struct represents details of the written bundle."] # [doc = ""] # [doc = " The index is present from [`FormatVersion::V1_1`] on."] # [allow (dead_code)] struct BinIndex { # [doc = " The number of 32-bit fields written in the index, including the field"] # [doc = " count."] field_count : u32 , # [doc = " The offset of the end of the key block in 32-bit values from the"] # [doc = " beginning of the body."] keys_end : u32 , # [doc = " The offset of the end of the resources block in 32-bit values from the"] # [doc = " beginning of the body."] resources_end : u32 , # [doc = " The offset of the end of the bundle in 32-bit values from the beginning"] # [doc = " of the body."] # [doc = ""] # [doc = " In all versions through [`FormatVersion::V3_0`], this is always the same"] # [doc = " as `resources_end`."] bundle_end : u32 , # [doc = " The number of entries in the largest table in the bundle."] largest_table_entry_count : u32 , # [doc = " Attributes describing resolution of external resources."] # [doc = ""] # [doc = " Present from [`FormatVersion::V1_2`] on."] bundle_attributes : Option < u32 > , # [doc = " The offset of the end of the 16-bit data block in 32-bit values from the"] # [doc = " beginning of the body."] # [doc = ""] # [doc = " Present from [`FormatVersion::V2_0`] on."] data_16_bit_end : Option < u32 > , # [doc = " The resource pool bundle checksum."] # [doc = ""] # [doc = " Present from [`FormatVersion::V2_0`] on when the bundle either is a pool"] # [doc = " bundle or uses a pool bundle for sharing resources."] pool_checksum : Option < u32 > , }
};
}
