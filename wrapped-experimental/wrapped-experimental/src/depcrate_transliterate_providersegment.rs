// Generated macro for Segment (struct)
macro_rules! Depcrate_transliterate_providerSegment {
() => {
// Module: crate::transliterate::provider
// Provides: {"Segment"}
// Dependencies: {}
# [doc = " Segments store matched parts of the input dynamically and can be referred to by back references"] # [doc = " in the replacer."] # [derive (Debug , Clone)] # [make_varule (SegmentULE)] # [zerovec :: skip_derive (Ord)] # [zerovec :: derive (Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize) , zerovec :: derive (Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize) , zerovec :: derive (Serialize))] pub struct Segment < 'a > { # [doc = " The 0-based index of this segment."] pub idx : u16 , # [cfg_attr (feature = "serde" , serde (borrow))] # [doc = " The content of the segment."] pub content : Cow < 'a , str > , }
};
}
