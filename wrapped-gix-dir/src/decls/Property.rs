macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! Property {
    () => {
        deps!();
        # [doc = " A way of attaching additional information to an [Entry] ."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , Ord , PartialOrd)] pub enum Property { # [doc = " The entry was named `.git`, matched according to the case-sensitivity rules of the repository."] DotGit , # [doc = " The entry is a directory, and that directory is empty."] EmptyDirectory , # [doc = " The entry is a directory, it is empty and the current working directory."] # [doc = ""] # [doc = " The caller should pay special attention to this very special case, as it is indeed only possible to run into it"] # [doc = " while traversing the directory for deletion."] # [doc = " Non-empty directory will never be collapsed, hence if they are working directories, they naturally become unobservable."] EmptyDirectoryAndCWD , # [doc = " Always in conjunction with a directory on disk that is also known as cone-mode sparse-checkout exclude marker"] # [doc = " - i.e. a directory that is excluded, so its whole content is excluded and not checked out nor is part of the index."] # [doc = ""] # [doc = " Note that evne if the directory is empty, it will only have this state, not `EmptyDirectory`."] TrackedExcluded , }
    };
}

Property!()