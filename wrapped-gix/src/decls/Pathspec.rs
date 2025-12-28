macro_rules! deps {
    () => {
        Repository!();
        Note!();
        Clone!();
    };
}

macro_rules! Pathspec {
    () => {
        deps!();
        # [doc = " A utility to make matching against pathspecs simple."] # [doc = ""] # [doc = " Note that to perform pathspec matching, attribute access might need to be provided. For that, we use our own"] # [doc = " and argue that the implementation is only going to incur costs for it when a pathspec matches *and* has attributes."] # [doc = " Should this potential duplication of effort to maintain attribute state be unacceptable, the user may fall back"] # [doc = " to the underlying plumbing."] # [derive (Clone)] # [cfg (feature = "attributes")] pub struct Pathspec < 'repo > { # [doc = " The owning repository."] pub repo : & 'repo Repository , # [doc = " The cache to power attribute access. It's only initialized if we have a pattern with attributes."] pub (crate) stack : Option < gix_worktree :: Stack > , # [doc = " The prepared search to use for checking matches."] pub (crate) search : gix_pathspec :: Search , }
    };
}

Pathspec!();