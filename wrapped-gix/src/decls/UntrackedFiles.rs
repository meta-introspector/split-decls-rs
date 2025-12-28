macro_rules! deps {
    () => {
        Default!();
        Clone!();
    };
}

macro_rules! UntrackedFiles {
    () => {
        deps!();
        # [doc = " How untracked files should be handled."] # [derive (Default , Copy , Clone , Debug , Ord , PartialOrd , Eq , PartialEq , Hash)] pub enum UntrackedFiles { # [doc = " Do not show any untracked files."] # [doc = ""] # [doc = " This can mean no directory walk is performed."] None , # [doc = " If possible, collapse files into their parent folders to reduce the amount of"] # [doc = " emitted untracked files."] # [default] Collapsed , # [doc = " Show each individual untracked file or directory (if empty directories are emitted) that the dirwalk encountered ."] Files , }
    };
}

UntrackedFiles!()