macro_rules! deps {
    () => {
        Hir!();
    };
}

macro_rules! ErrorKind {
    () => {
        deps!();
        # [doc = " The type of an error that occurred while building an `Hir`."] # [doc = ""] # [doc = " This error type is marked as `non_exhaustive`. This means that adding a"] # [doc = " new variant is not considered a breaking change."] # [non_exhaustive] # [derive (Clone , Debug , Eq , PartialEq)] pub enum ErrorKind { # [doc = " This error occurs when a Unicode feature is used when Unicode"] # [doc = " support is disabled. For example `(?-u:\\pL)` would trigger this error."] UnicodeNotAllowed , # [doc = " This error occurs when translating a pattern that could match a byte"] # [doc = " sequence that isn't UTF-8 and `utf8` was enabled."] InvalidUtf8 , # [doc = " This error occurs when one uses a non-ASCII byte for a line terminator,"] # [doc = " but where Unicode mode is enabled and UTF-8 mode is disabled."] InvalidLineTerminator , # [doc = " This occurs when an unrecognized Unicode property name could not"] # [doc = " be found."] UnicodePropertyNotFound , # [doc = " This occurs when an unrecognized Unicode property value could not"] # [doc = " be found."] UnicodePropertyValueNotFound , # [doc = " This occurs when a Unicode-aware Perl character class (`\\w`, `\\s` or"] # [doc = " `\\d`) could not be found. This can occur when the `unicode-perl`"] # [doc = " crate feature is not enabled."] UnicodePerlClassNotFound , # [doc = " This occurs when the Unicode simple case mapping tables are not"] # [doc = " available, and the regular expression required Unicode aware case"] # [doc = " insensitivity."] UnicodeCaseUnavailable , }
    };
}

ErrorKind!()