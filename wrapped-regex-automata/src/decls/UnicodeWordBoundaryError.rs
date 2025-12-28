macro_rules! UnicodeWordBoundaryError {
    () => {
        # [doc = " An error that occurs when the Unicode-aware `\\w` class is unavailable."] # [doc = ""] # [doc = " This error can occur when the data tables necessary for the Unicode aware"] # [doc = " Perl character class `\\w` are unavailable. The `\\w` class is used to"] # [doc = " determine whether a codepoint is considered a word character or not when"] # [doc = " determining whether a Unicode aware `\\b` (or `\\B`) matches at a particular"] # [doc = " position."] # [doc = ""] # [doc = " This error can only occur when the `unicode-word-boundary` feature is"] # [doc = " disabled."] # [derive (Clone , Debug)] pub struct UnicodeWordBoundaryError (()) ;
    };
}

UnicodeWordBoundaryError!();