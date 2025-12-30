// Generated macro for impl_1025 (impl)
macro_rules! Depcrate_transliterate_transliterator_replaceableimpl_1025 {
() => {
// Module: crate::transliterate::transliterator::replaceable
// Provides: {"impl_1025"}
// Dependencies: {}
impl Debug for Replaceable < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "{:?}" , self . content . hidden_prefix ()) ? ; write ! (f , "[[[") ? ; write ! (f , "{}" , & self . as_str () [.. self . freeze_pre_len]) ? ; write ! (f , "{{{{{{") ? ; write ! (f , "{}" , & self . as_str () [self . freeze_pre_len .. self . cursor ()]) ? ; write ! (f , "|||") ? ; write ! (f , "{}" , & self . as_str () [self . cursor () .. self . allowed_upper_bound ()]) ? ; write ! (f , "}}}}}}") ? ; write ! (f , "{}" , & self . as_str () [self . allowed_upper_bound () ..]) ? ; write ! (f , "]]]") ? ; write ! (f , "{:?}" , self . content . hidden_suffix ()) ? ; Ok (()) } }
};
}
