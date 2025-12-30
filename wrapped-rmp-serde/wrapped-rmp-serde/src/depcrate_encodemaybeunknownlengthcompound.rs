// Generated macro for MaybeUnknownLengthCompound (struct)
macro_rules! Depcrate_encodeMaybeUnknownLengthCompound {
() => {
// Module: crate::encode
// Provides: {"MaybeUnknownLengthCompound"}
// Dependencies: {}
# [doc = " Contains a `Serializer` for encoding elements of sequences and maps."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " If , for example, a field inside a struct is tagged with `#serde(flatten)` the total number of"] # [doc = " fields of this struct will be unknown to serde because flattened fields may have name clashes"] # [doc = " and then will be overwritten. So, serde wants to serialize the struct as a map with an unknown"] # [doc = " length."] # [doc = ""] # [doc = " For the described case a `UnknownLengthCompound` is used to encode the elements. On `end()`"] # [doc = " the counted length and the encoded elements will be written to the `Serializer`. A caveat is,"] # [doc = " that structs that contain flattened fields arem always written as a map, even when compact"] # [doc = " representaion is desired."] # [doc = ""] # [doc = " Otherwise, if the length is known, the elements will be encoded directly by the `Serializer`."] # [derive (Debug)] # [doc (hidden)] pub struct MaybeUnknownLengthCompound < 'a , W , C > { se : & 'a mut Serializer < W , C > , compound : Option < UnknownLengthCompound > , }
};
}
