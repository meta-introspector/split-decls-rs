// Generated macro for inner_range (function)
macro_rules! Depcrate_cstrinner_range {
() => {
// Module: crate::cstr
// Provides: {"inner_range"}
// Dependencies: {}
# [doc = " The range within `self.raw` that excludes the quotes and potential `r#`."] fn inner_range (num_hashes : Option < u8 > , start_suffix : usize) -> Range < usize > { match num_hashes { None => 2 .. start_suffix - 1 , Some (n) => 2 + n as usize + 1 .. start_suffix - n as usize - 1 , } }
};
}
