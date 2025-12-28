macro_rules! deps {
    () => {
        Pattern!();
    };
}

macro_rules! macro_1 {
    () => {
        deps!();
        bitflags ! { # [doc = " Information about a [`Pattern`]."] # [doc = ""] # [doc = " Its main purpose is to accelerate pattern matching, or to negate the match result or to"] # [doc = " keep special rules only applicable when matching paths."] # [doc = ""] # [doc = " The mode is typically created when parsing the pattern by inspecting it and isn't typically handled by the user."] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [derive (Debug , PartialEq , Eq , Hash , Copy , Clone , Ord , PartialOrd)] pub struct Mode : u32 { # [doc = " The pattern does not contain a sub-directory and - it doesn't contain slashes after removing the trailing one."] const NO_SUB_DIR = 1 << 0 ; # [doc = " A pattern that is '*literal', meaning that it ends with what's given here"] const ENDS_WITH = 1 << 1 ; # [doc = " The pattern must match a directory, and not a file."] const MUST_BE_DIR = 1 << 2 ; # [doc = " The pattern matches, but should be negated. Note that this mode has to be checked and applied by the caller."] const NEGATIVE = 1 << 3 ; # [doc = " The pattern starts with a slash and thus matches only from the beginning."] const ABSOLUTE = 1 << 4 ; } }
    };
}

macro_1!();