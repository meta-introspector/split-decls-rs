macro_rules! Pattern {
    () => {
        # [doc = " A glob pattern optimized for matching paths relative to a root directory."] # [doc = ""] # [doc = " For normal globbing, use [`wildmatch()`] instead."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Pattern { # [doc = " the actual pattern bytes"] pub text : BString , # [doc = " Additional information to help accelerate pattern matching."] pub mode : pattern :: Mode , # [doc = " The position in `text` with the first wildcard character, or `None` if there is no wildcard at all."] pub first_wildcard_pos : Option < usize > , }
    };
}

Pattern!()