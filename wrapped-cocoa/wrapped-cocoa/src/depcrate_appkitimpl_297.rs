// Generated macro for impl_297 (impl)
macro_rules! Depcrate_appkitimpl_297 {
() => {
// Module: crate::appkit
// Provides: {"impl_297"}
// Dependencies: {}
impl NSSpellChecker for id { unsafe fn sharedSpellChecker (_ : Self) -> id { msg_send ! [class ! (NSSpellChecker) , sharedSpellChecker] } unsafe fn checkSpellingOfString_startingAt (self , stringToCheck : id , startingOffset : NSInteger ,) -> NSRange { msg_send ! [self , checkSpellingOfString : stringToCheck startingAt : startingOffset] } unsafe fn checkSpellingOfString_startingAt_language_wrap_inSpellDocumentWithTag_wordCount (self , stringToCheck : id , startingOffset : NSInteger , language : id , wrapFlag : BOOL , tag : NSInteger ,) -> (NSRange , NSInteger) { let mut wordCount = 0 ; let range = msg_send ! [self , checkSpellingOfString : stringToCheck startingAt : startingOffset language : language wrap : wrapFlag inSpellDocumentWithTag : tag wordCount :& mut wordCount] ; (range , wordCount) } unsafe fn uniqueSpellDocumentTag (_ : Self) -> NSInteger { msg_send ! [class ! (NSSpellChecker) , uniqueSpellDocumentTag] } unsafe fn closeSpellDocumentWithTag (self , tag : NSInteger) { msg_send ! [self , closeSpellDocumentWithTag : tag] } unsafe fn ignoreWord_inSpellDocumentWithTag (self , wordToIgnore : id , tag : NSInteger) { msg_send ! [self , ignoreWord : wordToIgnore inSpellDocumentWithTag : tag] } }
};
}
