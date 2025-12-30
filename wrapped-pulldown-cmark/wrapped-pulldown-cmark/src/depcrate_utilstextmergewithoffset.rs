// Generated macro for TextMergeWithOffset (struct)
macro_rules! Depcrate_utilsTextMergeWithOffset {
() => {
// Module: crate::utils
// Provides: {"TextMergeWithOffset"}
// Dependencies: {}
# [doc = " Merge consecutive `Event::Text` events into only one, with offsets."] # [doc = ""] # [doc = " Compatible with with [`OffsetIter`](crate::OffsetIter)."] # [derive (Debug)] pub struct TextMergeWithOffset < 'a , I > { iter : I , last_event : Option < (Event < 'a > , Range < usize >) > , }
};
}
