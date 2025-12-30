// Generated macro for impl_49 (impl)
macro_rules! Depcrate_generatedimpl_49 {
() => {
// Module: crate::generated
// Provides: {"impl_49"}
// Dependencies: {}
impl LSMText { # [doc = " Creates a new text."] # [doc (alias = "LSMTextCreate")] # [inline] pub unsafe fn new (alloc : Option < & CFAllocator > , mapref : & LSMMap) -> CFRetained < LSMText > { extern "C-unwind" { fn LSMTextCreate (alloc : Option < & CFAllocator > , mapref : & LSMMap ,) -> Option < NonNull < LSMText > > ; } let ret = unsafe { LSMTextCreate (alloc , mapref) } ; let ret = ret . expect ("function was marked as returning non-null, but actually returned NULL") ; unsafe { CFRetained :: from_raw (ret) } } # [doc = " Adds a word to the text. The order of words is significant if the map"] # [doc = " uses pairs or triplets, and the count of words is always significant."] # [doc (alias = "LSMTextAddWord")] # [inline] pub unsafe fn add_word (& self , word : & CFString) -> OSStatus { extern "C-unwind" { fn LSMTextAddWord (textref : & LSMText , word : & CFString) -> OSStatus ; } unsafe { LSMTextAddWord (self , word) } } # [doc = " Breaks a string into words using the locale provided and adds the words"] # [doc = " to the text."] # [doc (alias = "LSMTextAddWords")] # [inline] pub unsafe fn add_words (& self , words : & CFString , locale : Option < & CFLocale > , flags : CFOptionFlags ,) -> OSStatus { extern "C-unwind" { fn LSMTextAddWords (textref : & LSMText , words : & CFString , locale : Option < & CFLocale > , flags : CFOptionFlags ,) -> OSStatus ; } unsafe { LSMTextAddWords (self , words , locale , flags) } } }
};
}
