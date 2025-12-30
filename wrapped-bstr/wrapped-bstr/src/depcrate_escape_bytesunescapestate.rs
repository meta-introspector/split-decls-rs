// Generated macro for UnescapeState (enum)
macro_rules! Depcrate_escape_bytesUnescapeState {
() => {
// Module: crate::escape_bytes
// Provides: {"UnescapeState"}
// Dependencies: {}
# [doc = " The state used by the FSM in the unescaping iterator."] # [derive (Clone , Debug)] # [cfg (feature = "alloc")] enum UnescapeState { # [doc = " The start state. Look for an escape sequence, otherwise emit the next"] # [doc = " codepoint as-is."] Start , # [doc = " Emit the byte at `buf[cur]`."] # [doc = ""] # [doc = " This state should never be created when `cur >= len`. That is, when"] # [doc = " this state is visited, it is assumed that `cur < len`."] Bytes { buf : [u8 ; 11] , cur : usize , len : usize } , # [doc = " This state is entered after a `\\` is seen."] Escape , # [doc = " This state is entered after a `\\x` is seen."] HexFirst , # [doc = " This state is entered after a `\\xN` is seen, where `N` is in"] # [doc = " `[0-9A-Fa-f]`. The given codepoint corresponds to `N`."] HexSecond (char) , }
};
}
