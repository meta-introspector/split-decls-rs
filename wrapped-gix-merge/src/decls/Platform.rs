macro_rules! deps {
    () => {
        Options!();
        Resource!();
        Driver!();
        Mode!();
        Outcome!();
        Pipeline!();
    };
}

macro_rules! Platform {
    () => {
        deps!();
        # [doc = " A utility for gathering and processing all state necessary to perform a three-way merge."] # [doc = ""] # [doc = " It can re-use buffers if all three parts of participating in the merge are"] # [doc = " set repeatedly."] # [derive (Clone)] pub struct Platform { # [doc = " The current version (ours)."] current : Option < platform :: Resource > , # [doc = " The ancestor version (base)."] ancestor : Option < platform :: Resource > , # [doc = " The other version (theirs)."] other : Option < platform :: Resource > , # [doc = " A way to convert objects into a diff-able format."] pub filter : Pipeline , # [doc = " A way to access `.gitattributes`"] pub attr_stack : gix_worktree :: Stack , # [doc = " Further configuration that affects the merge."] pub options : platform :: Options , # [doc = " All available merge drivers."] # [doc = ""] # [doc = " They are referenced in git-attributes by name, and we hand out indices into this array."] drivers : Vec < Driver > , # [doc = " Pre-configured attributes to obtain additional merge-related information."] attrs : gix_filter :: attributes :: search :: Outcome , # [doc = " The way we convert resources into mergeable states."] pub filter_mode : pipeline :: Mode , }
    };
}

Platform!()