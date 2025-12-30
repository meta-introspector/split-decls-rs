// Generated macro for update_str (function)
macro_rules! Depcrate_rawupdate_str {
() => {
// Module: crate::raw
// Provides: {"update_str"}
// Dependencies: {}
# [doc = " Uses the null-terminated string `name` as key to the _MALLCTL NAMESPACE_ and"] # [doc = " writes its `value` returning its previous value."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is unsafe because if the key does not return a pointer to a"] # [doc = " null-terminated string the behavior is undefined."] # [doc = ""] # [doc = " For example, a key for a `u64` value can be used to read a pointer on 64-bit"] # [doc = " platform, where this pointer will point to the address denoted by the `u64`s"] # [doc = " representation. Also, a key to a `*mut extent_hooks_t` will return a pointer"] # [doc = " that will not point to a null-terminated string."] # [doc = ""] # [doc = " This function needs to compute the length of the string by looking for the"] # [doc = " null-terminator: `\\0`. This requires reading the memory behind the pointer."] # [doc = ""] # [doc = " If the pointer is invalid (e.g. because it was converted from a `u64` that"] # [doc = " does not represent a valid address), reading the string to look for `\\0`"] # [doc = " will dereference a non-dereferenceable pointer, which is undefined behavior."] # [doc = ""] # [doc = " If the pointer is valid but it does not point to a null-terminated string,"] # [doc = " looking for `\\0` will read garbage and might end up reading out-of-bounds,"] # [doc = " which is undefined behavior."] pub unsafe fn update_str (name : & [u8] , value : & 'static [u8] ,) -> Result < & 'static [u8] > { let ptr : * const c_char = update (name , value . as_ptr () as * const c_char) ? ; Ok (ptr2str (ptr)) }
};
}
