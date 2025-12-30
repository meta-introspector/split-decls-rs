// Generated macro for Commit (enum)
macro_rules! Depcrate_errorCommit {
() => {
// Module: crate::error
// Provides: {"Commit"}
// Dependencies: {}
# [doc = " Enum used to indicate if a parser committed any items of the stream it was given as an input."] # [doc = ""] # [doc = " This is used by parsers such as `or` and `choice` to determine if they should try to parse"] # [doc = " with another parser as they will only be able to provide good error reporting if the preceding"] # [doc = " parser did not commit to the parse."] # [derive (Clone , PartialEq , Debug , Copy)] pub enum Commit < T > { # [doc = " Constructor indicating that the parser has committed to this parse. If a parser after this fails,"] # [doc = " other parser alternatives will not be attempted (`CommitErr` will be returned)"] Commit (T) , # [doc = " Constructor indicating that the parser has not committed to this parse. If a parser after this fails,"] # [doc = " other parser alternatives will be attempted (`EmptyErr` will be returned)"] Peek (T) , }
};
}
