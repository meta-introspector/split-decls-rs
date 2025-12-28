macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! error {
    () => {
        deps!();
        mod error { use bstr :: BString ; # [doc = " The error returned by [`parse::Lines`][crate::parse::Lines]."] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error (r"Line {line_number} has a negative pattern, for literal characters use \!: {line}")] PatternNegation { line_number : usize , line : BString } , # [error ("Attribute in line {line_number} has non-ascii characters or starts with '-': {attribute}")] AttributeName { line_number : usize , attribute : BString } , # [error ("Macro in line {line_number} has non-ascii characters or starts with '-': {macro_name}")] MacroName { line_number : usize , macro_name : BString } , # [error ("Could not unquote attributes line")] Unquote (# [from] gix_quote :: ansi_c :: undo :: Error) , } }
    };
}

error!()