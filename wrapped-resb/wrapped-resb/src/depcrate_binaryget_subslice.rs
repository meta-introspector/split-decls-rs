// Generated macro for get_subslice (function)
macro_rules! Depcrate_binaryget_subslice {
() => {
// Module: crate::binary
// Provides: {"get_subslice"}
// Dependencies: {}
# [doc = " Gets a subset of the given `u8` slice based on the specified index with"] # [doc = " bounds checking."] # [doc = ""] # [doc = " Returns the subslice. Returns an error if the index is not valid for the"] # [doc = " input."] fn get_subslice < I > (input : & [u8] , index : I) -> Result < & [u8] , BinaryDeserializerError > where I : SliceIndex < [u8] , Output = [u8] > , { input . get (index) . ok_or (BinaryDeserializerError :: invalid_data ("unexpected end of input" ,)) }
};
}
