macro_rules! Case {
    () => {
        # [doc = " Describes whether to match a path case sensitively or not."] # [doc = ""] # [doc = " Used in [`Pattern::matches_repo_relative_path()`]."] # [derive (Default , Debug , PartialOrd , PartialEq , Copy , Clone , Hash , Ord , Eq)] pub enum Case { # [doc = " The case affects the match"] # [default] Sensitive , # [doc = " Ignore the case of ascii characters."] Fold , }
    };
}

Case!();