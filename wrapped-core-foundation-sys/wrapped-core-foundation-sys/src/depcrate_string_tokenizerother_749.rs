// Generated macro for other_749 (other)
macro_rules! Depcrate_string_tokenizerother_749 {
() => {
// Module: crate::string_tokenizer
// Provides: {"other_749"}
// Dependencies: {}
unsafe extern "C" { pub fn CFStringTokenizerCreate (alloc : CFAllocatorRef , string : CFStringRef , range : CFRange , options : CFOptionFlags , locale : CFLocaleRef ,) -> CFStringTokenizerRef ; pub fn CFStringTokenizerSetString (tokenizer : CFStringTokenizerRef , string : CFStringRef , range : CFRange ,) ; pub fn CFStringTokenizerAdvanceToNextToken (tokenizer : CFStringTokenizerRef ,) -> CFStringTokenizerTokenType ; pub fn CFStringTokenizerGoToTokenAtIndex (tokenizer : CFStringTokenizerRef , index : CFIndex ,) -> CFStringTokenizerTokenType ; pub fn CFStringTokenizerCopyCurrentTokenAttribute (tokenizer : CFStringTokenizerRef , attribute : CFOptionFlags ,) -> CFTypeRef ; pub fn CFStringTokenizerGetCurrentTokenRange (tokenizer : CFStringTokenizerRef) -> CFRange ; pub fn CFStringTokenizerGetCurrentSubTokens (tokenizer : CFStringTokenizerRef , ranges : * mut CFRange , maxRangeLength : CFIndex , derivedSubTokens : CFMutableArrayRef ,) -> CFIndex ; pub fn CFStringTokenizerCopyBestStringLanguage (string : CFStringRef , range : CFRange ,) -> CFStringRef ; pub fn CFStringTokenizerGetTypeID () -> CFTypeID ; }
};
}
