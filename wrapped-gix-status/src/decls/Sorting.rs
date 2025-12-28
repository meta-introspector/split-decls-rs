macro_rules! Sorting {
    () => {
        # [doc = " The way all output should be sorted."] # [derive (Clone , Copy , Default , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub enum Sorting { # [doc = " The entries are sorted by their path in a case-sensitive fashion."] # [default] ByPathCaseSensitive , }
    };
}

Sorting!();