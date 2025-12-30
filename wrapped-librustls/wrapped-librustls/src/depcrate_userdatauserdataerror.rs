// Generated macro for UserdataError (enum)
macro_rules! Depcrate_userdataUserdataError {
() => {
// Module: crate::userdata
// Provides: {"UserdataError"}
// Dependencies: {}
# [derive (Clone , Debug)] pub (crate) enum UserdataError { # [doc = " try_pop was called twice."] AlreadyPopped , # [doc = " The RefCell is borrowed somewhere else."] AlreadyBorrowed , # [doc = " The stack of userdata items was already empty."] EmptyStack , # [doc = " The LocalKey was destroyed before this call."] # [doc = " See <https://doc.rust-lang.org/std/thread/struct.LocalKey.html#method.try_with>"] AccessError , # [doc = " Unexpected pointer when popping."] WrongData , }
};
}
