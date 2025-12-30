// Generated macro for impl_635 (impl)
macro_rules! Depcrate_strings_mutf8_charsimpl_635 {
() => {
// Module: crate::strings::mutf8_chars
// Provides: {"impl_635"}
// Dependencies: {}
impl < 'local , StringRef > Drop for MUTF8Chars < 'local , StringRef > where StringRef : AsRef < JString < 'local > > + Reference , { fn drop (& mut self) { unsafe fn release_string_utf_chars (obj : jni_sys :: jobject , chars : * const c_char ,) -> Result < () > { JavaVM :: singleton () ? . with_top_local_frame (| env | { jni_call_unchecked ! (env , v1_1 , ReleaseStringUTFChars , obj , chars) ; Ok (()) }) } match unsafe { release_string_utf_chars (self . obj . as_raw () , self . chars) } { Ok (()) => { } Err (e) => warn ! ("error dropping java str: {}" , e) , } } }
};
}
