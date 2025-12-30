// Generated macro for CodeBlockKind (enum)
macro_rules! DepcrateCodeBlockKind {
() => {
// Module: crate
// Provides: {"CodeBlockKind"}
// Dependencies: {}
# [doc = " Codeblock kind."] # [derive (Clone , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub enum CodeBlockKind < 'a > { Indented , # [doc = " The value contained in the tag describes the language of the code, which may be empty."] # [cfg_attr (feature = "serde" , serde (borrow))] Fenced (CowStr < 'a >) , }
};
}
