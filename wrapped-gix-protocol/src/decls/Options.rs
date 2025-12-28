macro_rules! deps {
    () => {
        Tags!();
        Shallow!();
    };
}

macro_rules! Options {
    () => {
        deps!();
        # [doc = " Options for use in [`fetch()`](`crate::fetch()`)"] # [derive (Debug , Clone)] pub struct Options < 'a > { # [doc = " The path to the file containing the shallow commit boundary."] # [doc = ""] # [doc = " When needed, it will be locked in preparation for being modified."] pub shallow_file : PathBuf , # [doc = " How to deal with shallow repositories. It does affect how negotiations are performed."] pub shallow : & 'a Shallow , # [doc = " Describe how to handle tags when fetching."] pub tags : Tags , # [doc = " If `true`, if we fetch from a remote that only offers shallow clones, the operation will fail with an error"] # [doc = " instead of writing the shallow boundary to the shallow file."] pub reject_shallow_remote : bool , }
    };
}

Options!()