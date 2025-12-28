macro_rules! AbbreviationsCacheStrategy {
    () => {
        # [doc = " The strategy to use for caching abbreviations."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [non_exhaustive] pub enum AbbreviationsCacheStrategy { # [doc = " Cache abbreviations that are used more than once."] # [doc = ""] # [doc = " This is useful if the units in the `.debug_info` section will be parsed only once."] Duplicates , # [doc = " Cache all abbreviations."] # [doc = ""] # [doc = " This is useful if the units in the `.debug_info` section will be parsed more than once."] All , }
    };
}

AbbreviationsCacheStrategy!()