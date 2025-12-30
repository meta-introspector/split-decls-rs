// Generated macro for UnknownLengthCompound (struct)
macro_rules! Depcrate_encodeUnknownLengthCompound {
() => {
// Module: crate::encode
// Provides: {"UnknownLengthCompound"}
// Dependencies: {}
# [doc = " Contains a `Serializer` for sequences and maps whose length is not yet known"] # [doc = " and a counter for the number of elements that are encoded by the `Serializer`."] # [derive (Debug)] struct UnknownLengthCompound { se : Serializer < Vec < u8 > , DefaultConfig > , elem_count : u32 , }
};
}
