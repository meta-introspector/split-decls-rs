// Generated macro for NSSpellChecker (trait)
macro_rules! Depcrate_appkitNSSpellChecker {
() => {
// Module: crate::appkit
// Provides: {"NSSpellChecker"}
// Dependencies: {}
pub trait NSSpellChecker : Sized { unsafe fn sharedSpellChecker (_ : Self) -> id ; unsafe fn checkSpellingOfString_startingAt (self , stringToCheck : id , startingOffset : NSInteger ,) -> NSRange ; unsafe fn checkSpellingOfString_startingAt_language_wrap_inSpellDocumentWithTag_wordCount (self , stringToCheck : id , startingOffset : NSInteger , language : id , wrapFlag : BOOL , tag : NSInteger ,) -> (NSRange , NSInteger) ; unsafe fn uniqueSpellDocumentTag (_ : Self) -> NSInteger ; unsafe fn closeSpellDocumentWithTag (self , tag : NSInteger) ; unsafe fn ignoreWord_inSpellDocumentWithTag (self , wordToIgnore : id , tag : NSInteger) ; }
};
}
