// Generated macro for failed_creating_dictionary (function)
macro_rules! Depcrate_dictionaryfailed_creating_dictionary {
() => {
// Module: crate::dictionary
// Provides: {"failed_creating_dictionary"}
// Dependencies: {}
# [doc = " Roughly same as `failed_creating_array`."] # [cold] fn failed_creating_dictionary (len : CFIndex) -> ! { # [cfg (feature = "alloc")] { use alloc :: alloc :: { handle_alloc_error , Layout } ; use core :: mem :: align_of ; let layout = Layout :: array :: < (* const () , * const ()) > (len as usize) . unwrap_or_else (| _ | unsafe { Layout :: from_size_align_unchecked (0 , align_of :: < * const () > ()) }) ; handle_alloc_error (layout) } # [cfg (not (feature = "alloc"))] { panic ! ("failed allocating CFDictionary holding {len} elements") } }
};
}
