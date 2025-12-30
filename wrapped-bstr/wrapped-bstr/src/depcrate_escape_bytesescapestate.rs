// Generated macro for EscapeState (enum)
macro_rules! Depcrate_escape_bytesEscapeState {
() => {
// Module: crate::escape_bytes
// Provides: {"EscapeState"}
// Dependencies: {}
# [doc = " The state used by the FSM in the escaping iterator."] # [derive (Clone , Debug)] enum EscapeState { # [doc = " Read and remove the next byte from 'remaining'. If 'remaining' is"] # [doc = " empty, then return None. Otherwise, escape the byte according to the"] # [doc = " following rules or emit it as-is."] # [doc = ""] # [doc = " If it's \\n, \\r, \\t, \\\\ or \\0, then emit a '\\' and set the current"] # [doc = " state to 'SpecialEscape(n | r | t | \\ | 0)'. Otherwise, if the 'byte'"] # [doc = " is not in [\\x21-\\x5B\\x5D-\\x7E], then emit a '\\' and set the state to"] # [doc = " to 'HexEscapeX(byte)'."] Start , # [doc = " Emit the given codepoint as is. This assumes '\\' has just been emitted."] # [doc = " Then set the state to 'Start'."] SpecialEscape (char) , # [doc = " Emit the 'x' part of a hex escape. This assumes '\\' has just been"] # [doc = " emitted. Then set the state to 'HexEscapeHighNybble(byte)'."] HexEscapeX (u8) , # [doc = " Emit the high nybble of the byte as a hexadecimal digit. This"] # [doc = " assumes '\\x' has just been emitted. Then set the state to"] # [doc = " 'HexEscapeLowNybble(byte)'."] HexEscapeHighNybble (u8) , # [doc = " Emit the low nybble of the byte as a hexadecimal digit. This assume"] # [doc = " '\\xZ' has just been emitted, where 'Z' is the high nybble of this byte."] # [doc = " Then set the state to 'Start'."] HexEscapeLowNybble (u8) , }
};
}
