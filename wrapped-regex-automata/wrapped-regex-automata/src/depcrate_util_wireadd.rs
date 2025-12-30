// Generated macro for add (function)
macro_rules! Depcrate_util_wireadd {
() => {
// Module: crate::util::wire
// Provides: {"add"}
// Dependencies: {}
# [doc = " Add the given numbers, and on overflow, return an error that includes"] # [doc = " 'what' in the error message."] # [doc = ""] # [doc = " This is useful when doing arithmetic with untrusted data."] pub (crate) fn add (a : usize , b : usize , what : & 'static str ,) -> Result < usize , DeserializeError > { match a . checked_add (b) { Some (c) => Ok (c) , None => Err (DeserializeError :: arithmetic_overflow (what)) , } }
};
}
