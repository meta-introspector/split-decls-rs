// Generated macro for Hide (struct)
macro_rules! Depcrate_transliterate_transliterator_replaceableHide {
() => {
// Module: crate::transliterate::transliterator::replaceable
// Provides: {"Hide"}
// Dependencies: {}
# [doc = " A wrapper over `Vec<u8>` that only allows access (and modification) to a certain range."] # [doc = ""] # [doc = " This is useful for other types that might only need a part of a `Vec<u8>` to be of a certain"] # [doc = " structure, such as UTF-8. With this wrapper, they can assert their invariants only for the"] # [doc = " accessible portion."] # [doc = ""] # [doc = " All methods that take indices as arguments expect them to be relative to the *visible* part."] struct Hide < 'a > { raw : & 'a mut Vec < u8 > , hide_pre_len : usize , hide_post_len : usize , }
};
}
