// Generated macro for Item (enum)
macro_rules! Depcrate_formatItem {
() => {
// Module: crate::format
// Provides: {"Item"}
// Dependencies: {}
# [doc = " A single formatting item. This is used for both formatting and parsing."] # [derive (Clone , PartialEq , Eq , Debug , Hash)] pub enum Item < 'a > { # [doc = " A literally printed and parsed text."] Literal (& 'a str) , # [doc = " Same as `Literal` but with the string owned by the item."] # [cfg (feature = "alloc")] OwnedLiteral (Box < str >) , # [doc = " Whitespace. Prints literally but reads zero or more whitespace."] Space (& 'a str) , # [doc = " Same as `Space` but with the string owned by the item."] # [cfg (feature = "alloc")] OwnedSpace (Box < str >) , # [doc = " Numeric item. Can be optionally padded to the maximal length (if any) when formatting;"] # [doc = " the parser simply ignores any padded whitespace and zeroes."] Numeric (Numeric , Pad) , # [doc = " Fixed-format item."] Fixed (Fixed) , # [doc = " Issues a formatting error. Used to signal an invalid format string."] Error , }
};
}
