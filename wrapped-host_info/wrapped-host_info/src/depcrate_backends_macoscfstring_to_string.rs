// Generated macro for cfstring_to_string (function)
macro_rules! Depcrate_backends_macoscfstring_to_string {
() => {
// Module: crate::backends::macos
// Provides: {"cfstring_to_string"}
// Dependencies: {}
# [doc = " Converts a CFStringRef to a Rust String."] # [doc = " Returns None if the CFStringRef is null or conversion fails."] fn cfstring_to_string (cf_string : CFStringRef) -> Option < String > { if cf_string . is_null () { return None ; } unsafe { let direct_ptr = CFStringGetCStringPtr (cf_string , kCFStringEncodingUTF8) ; if ! direct_ptr . is_null () { return CStr :: from_ptr (direct_ptr as * const c_char) . to_str () . ok () . map (str :: to_owned) ; } let length = core_foundation_sys :: string :: CFStringGetLength (cf_string) ; let max_size = CFStringGetMaximumSizeForEncoding (length , kCFStringEncodingUTF8) + 1 ; const STACK_BUFFER_SIZE : usize = 256 ; let mut stack_buffer = [0u8 ; STACK_BUFFER_SIZE] ; if max_size <= STACK_BUFFER_SIZE as isize { let success = CFStringGetCString (cf_string , stack_buffer . as_mut_ptr () as * mut i8 , STACK_BUFFER_SIZE as isize , kCFStringEncodingUTF8 ,) ; if success != 0 { return CStr :: from_ptr (stack_buffer . as_ptr () as * const c_char) . to_str () . ok () . map (str :: to_owned) ; } } else { let mut heap_buffer = vec ! [0u8 ; max_size as usize] ; let success = CFStringGetCString (cf_string , heap_buffer . as_mut_ptr () as * mut i8 , max_size , kCFStringEncodingUTF8 ,) ; if success != 0 { return CStr :: from_ptr (heap_buffer . as_ptr () as * const c_char) . to_str () . ok () . map (str :: to_owned) ; } } None } }
};
}
