// Generated macro for Operation (enum)
macro_rules! DepcrateOperation {
() => {
// Module: crate
// Provides: {"Operation"}
// Dependencies: {}
# [doc = " A record of what kind of operation is happening that we should generate a token for."] # [derive (Serialize , Deserialize , Clone , Debug , PartialEq , Eq)] # [non_exhaustive] # [serde (tag = "operation" , rename_all = "kebab-case")] pub enum Operation < 'a > { # [doc = " The user is attempting to fetch a crate."] Read , # [doc = " The user is attempting to publish a crate."] Publish { # [doc = " The name of the crate"] name : & 'a str , # [doc = " The version of the crate"] vers : & 'a str , # [doc = " The checksum of the crate file being uploaded"] cksum : & 'a str , } , # [doc = " The user is attempting to yank a crate."] Yank { # [doc = " The name of the crate"] name : & 'a str , # [doc = " The version of the crate"] vers : & 'a str , } , # [doc = " The user is attempting to unyank a crate."] Unyank { # [doc = " The name of the crate"] name : & 'a str , # [doc = " The version of the crate"] vers : & 'a str , } , # [doc = " The user is attempting to modify the owners of a crate."] Owners { # [doc = " The name of the crate"] name : & 'a str , } , # [serde (other)] Unknown , }
};
}
