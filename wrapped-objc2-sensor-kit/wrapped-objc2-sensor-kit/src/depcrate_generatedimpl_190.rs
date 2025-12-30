// Generated macro for impl_190 (impl)
macro_rules! Depcrate_generatedimpl_190 {
() => {
// Module: crate::generated
// Provides: {"impl_190"}
// Dependencies: {}
# [doc = " SentimentCounts."] # [doc = ""] # [doc = " These metrics describe the number of words and emoji of a particular category typed during"] # [doc = " a keyboard session. Words and emoji may be counted in multiple categories."] impl SRKeyboardMetrics { extern_methods ! (# [doc = " The count of words typed per category in the session"] # [unsafe (method (wordCountForSentimentCategory :))] # [unsafe (method_family = none)] pub unsafe fn wordCountForSentimentCategory (& self , category : SRKeyboardMetricsSentimentCategory ,) -> NSInteger ; # [doc = " The count of emoji typed per category in the session"] # [unsafe (method (emojiCountForSentimentCategory :))] # [unsafe (method_family = none)] pub unsafe fn emojiCountForSentimentCategory (& self , category : SRKeyboardMetricsSentimentCategory ,) -> NSInteger ;) ; }
};
}
