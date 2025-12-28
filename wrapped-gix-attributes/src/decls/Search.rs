macro_rules! Search {
    () => {
        # [doc = " A grouping of lists of patterns while possibly keeping associated to their base path in order to find matches."] # [doc = ""] # [doc = " Pattern lists with base path are queryable relative to that base, otherwise they are relative to the repository root."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Default)] pub struct Search { # [doc = " A list of pattern lists, each representing a patterns from a file or specified by hand, in the order they were"] # [doc = " specified in."] # [doc = ""] # [doc = " When matching, this order is reversed."] patterns : Vec < gix_glob :: search :: pattern :: List < search :: Attributes > > , }
    };
}

Search!()