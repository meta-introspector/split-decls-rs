// Generated macro for Comment (struct)
macro_rules! Depcrate_parseComment {
() => {
// Module: crate::parse
// Provides: {"Comment"}
// Dependencies: {}
# [doc = " A parsed comment containing the comment marker and comment."] # [derive (Clone , Eq , PartialEq , Ord , PartialOrd , Hash , Debug , Default)] pub struct Comment < 'a > { # [doc = " The comment marker used. This is either a semicolon or octothorpe/hash."] pub tag : u8 , # [doc = " The parsed comment."] pub text : Cow < 'a , BStr > , }
};
}
