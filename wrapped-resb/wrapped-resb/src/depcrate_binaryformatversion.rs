// Generated macro for FormatVersion (enum)
macro_rules! Depcrate_binaryFormatVersion {
() => {
// Module: crate::binary
// Provides: {"FormatVersion"}
// Dependencies: {}
# [doc = " A `FormatVersion` represents a specific binary file format used for"] # [doc = " representing resource bundles."] # [doc = ""] # [doc = " A partial [specification] of each format version is present in the ICU4C"] # [doc = " source code."] # [doc = ""] # [doc = " [specification]: https://github.com/unicode-org/icu/blob/main/icu4c/source/common/uresdata.h"] # [derive (Clone , Copy , Debug , PartialEq , PartialOrd)] # [repr (u32)] enum FormatVersion { V1_0 , V1_1 , V1_2 , V1_3 , V2_0 , V3_0 , }
};
}
