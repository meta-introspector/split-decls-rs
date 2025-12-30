// Generated macro for MessageRef (struct)
macro_rules! Depcrate_commitMessageRef {
() => {
// Module: crate::commit
// Provides: {"MessageRef"}
// Dependencies: {}
# [doc = " A parsed commit message that assumes a title separated from the body by two consecutive newlines."] # [doc = ""] # [doc = " Titles can have any amount of whitespace"] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct MessageRef < 'a > { # [doc = " The title of the commit, as separated from the body with two consecutive newlines. The newlines are not included."] # [cfg_attr (feature = "serde" , serde (borrow))] pub title : & 'a BStr , # [doc = " All bytes not consumed by the title, excluding the separating newlines."] # [doc = ""] # [doc = " The body is `None` if there was now title separation or the body was empty after the separator."] pub body : Option < & 'a BStr > , }
};
}
