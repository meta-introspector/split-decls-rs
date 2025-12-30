// Generated macro for Part (enum)
macro_rules! Depcrate_hyperlinkPart {
() => {
// Module: crate::hyperlink
// Provides: {"Part"}
// Dependencies: {}
# [doc = " A hyperlink format part."] # [doc = ""] # [doc = " A sequence of these corresponds to a complete format. (Not all sequences"] # [doc = " are valid.)"] # [derive (Clone , Debug , Eq , PartialEq)] enum Part { # [doc = " Static text."] # [doc = ""] # [doc = " We use `Vec<u8>` here (and more generally treat a format string as a"] # [doc = " sequence of bytes) because file paths may be arbitrary bytes. A rare"] # [doc = " case, but one for which there is no good reason to choke on."] Text (Vec < u8 >) , # [doc = " Variable for the hostname."] Host , # [doc = " Variable for a WSL path prefix."] WSLPrefix , # [doc = " Variable for the file path."] Path , # [doc = " Variable for the line number."] Line , # [doc = " Variable for the column number."] Column , }
};
}
