// Generated macro for __ns_string_static (macro)
macro_rules! Depcrate_macros_ns_string__ns_string_static {
() => {
// Module: crate::macros::ns_string
// Provides: {"__ns_string_static"}
// Dependencies: {}
# [doc (hidden)] # [cfg (all (target_vendor = "apple" , feature = "unstable-static-nsstring"))] # [macro_export] macro_rules ! __ns_string_static { ($ inp : ident) => { # [link_section = "__TEXT,__cstring,cstring_literals"] static ASCII : [u8 ; $ inp . len () + 1] = { let mut res : [u8 ; $ inp . len () + 1] = [0 ; $ inp . len () + 1] ; let mut i = 0 ; while i < $ inp . len () { res [i] = $ inp [i] ; i += 1 ; } res } ; const UTF16_FULL : (& [u16 ; $ inp . len ()] , usize) = { let mut out = [0u16 ; $ inp . len ()] ; let mut iter = $ crate :: __ns_macro_helpers :: EncodeUtf16Iter :: new ($ inp) ; let mut written = 0 ; while let Some ((state , chars)) = iter . next () { iter = state ; out [written] = chars . repr [0] ; written += 1 ; if chars . len > 1 { out [written] = chars . repr [1] ; written += 1 ; } } (& { out } , written) } ; # [link_section = "__TEXT,__ustring"] static UTF16 : [u16 ; UTF16_FULL . 1 + 1] = { let mut res : [u16 ; UTF16_FULL . 1 + 1] = [0 ; UTF16_FULL . 1 + 1] ; let mut i = 0 ; while i < UTF16_FULL . 1 { res [i] = UTF16_FULL . 0 [i] ; i += 1 ; } res } ; # [link_section = "__DATA,__cfstring"] static CFSTRING : $ crate :: __ns_macro_helpers :: CFConstString = unsafe { if $ crate :: __ns_macro_helpers :: is_ascii_no_nul ($ inp) { $ crate :: __ns_macro_helpers :: CFConstString :: new_ascii (&$ crate :: __ns_macro_helpers :: __CFConstantStringClassReference , & ASCII ,) } else { $ crate :: __ns_macro_helpers :: CFConstString :: new_utf16 (&$ crate :: __ns_macro_helpers :: __CFConstantStringClassReference , & UTF16 ,) } } ; } ; }
};
}
