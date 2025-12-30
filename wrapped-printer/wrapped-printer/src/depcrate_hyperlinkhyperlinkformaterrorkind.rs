// Generated macro for HyperlinkFormatErrorKind (enum)
macro_rules! Depcrate_hyperlinkHyperlinkFormatErrorKind {
() => {
// Module: crate::hyperlink
// Provides: {"HyperlinkFormatErrorKind"}
// Dependencies: {}
# [derive (Clone , Debug , Eq , PartialEq)] enum HyperlinkFormatErrorKind { # [doc = " This occurs when there are zero variables in the format."] NoVariables , # [doc = " This occurs when the {path} variable is missing."] NoPathVariable , # [doc = " This occurs when the {line} variable is missing, while the {column}"] # [doc = " variable is present."] NoLineVariable , # [doc = " This occurs when an unknown variable is used."] InvalidVariable (String) , # [doc = " The format doesn't start with a valid scheme."] InvalidScheme , # [doc = " This occurs when an unescaped `}` is found without a corresponding"] # [doc = " `{` preceding it."] InvalidCloseVariable , # [doc = " This occurs when a `{` is found without a corresponding `}` following"] # [doc = " it."] UnclosedVariable , }
};
}
