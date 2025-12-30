// Generated macro for SeqSerializer (struct)
macro_rules! Depcrate_serSeqSerializer {
() => {
// Module: crate::ser
// Provides: {"SeqSerializer"}
// Dependencies: {}
# [doc = " Serializer for numbered sequences"] # [doc = ""] # [doc = " This wrapper is present when we are outputting a sequence (numbered indices)."] # [doc = " Making this a separate type centralises the handling of sequences"] # [doc = " and ensures we don't have any call sites for `ser::SerializeSeq::serialize_element`"] # [doc = " that don't do the necessary work of `SeqSerializer::new`."] # [doc = ""] # [doc = " Existence of this wrapper implies that `.0.keys.last()` is"] # [doc = " `Some(SerKey::Seq(next_index))`."] pub (crate) struct SeqSerializer < 'a > (& 'a mut ConfigSerializer) ;
};
}
