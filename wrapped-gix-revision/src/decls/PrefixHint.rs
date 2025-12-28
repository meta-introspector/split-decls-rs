macro_rules! PrefixHint {
    () => {
        # [doc = " A hint to make disambiguation when looking up prefixes possible."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] pub enum PrefixHint < 'a > { # [doc = " The prefix must be a commit."] MustBeCommit , # [doc = " The prefix refers to a commit, anchored to a ref and a revision generation in its future."] DescribeAnchor { # [doc = " The name of the reference, like `v1.2.3` or `main`."] ref_name : & 'a BStr , # [doc = " The future generation of the commit we look for, with 0 meaning the commit is referenced by"] # [doc = " `ref_name` directly."] generation : usize , } , }
    };
}

PrefixHint!()