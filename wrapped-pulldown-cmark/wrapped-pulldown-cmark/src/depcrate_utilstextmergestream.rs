// Generated macro for TextMergeStream (struct)
macro_rules! Depcrate_utilsTextMergeStream {
() => {
// Module: crate::utils
// Provides: {"TextMergeStream"}
// Dependencies: {}
# [doc = " Merge consecutive `Event::Text` events into only one."] # [derive (Debug)] pub struct TextMergeStream < 'a , I > { inner : TextMergeWithOffset < 'a , DummyOffsets < I > > , }
};
}
