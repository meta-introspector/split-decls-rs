// Generated macro for shl (function)
macro_rules! Depcrate_util_wireshl {
() => {
// Module: crate::util::wire
// Provides: {"shl"}
// Dependencies: {}
# [doc = " Shift `a` left by `b`, and on overflow, return an error that includes"] # [doc = " 'what' in the error message."] # [doc = ""] # [doc = " This is useful when doing arithmetic with untrusted data."] pub (crate) fn shl (a : usize , b : usize , what : & 'static str ,) -> Result < usize , DeserializeError > { let amount = u32 :: try_from (b) . map_err (| _ | DeserializeError :: arithmetic_overflow (what)) ? ; match a . checked_shl (amount) { Some (c) => Ok (c) , None => Err (DeserializeError :: arithmetic_overflow (what)) , } }
};
}
