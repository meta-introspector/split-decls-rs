macro_rules! deps {
    () => {
        Commit!();
    };
}

macro_rules! Parents {
    () => {
        deps!();
        # [doc = " An iterator over the parent commits of a commit."] # [doc = ""] # [doc = " Aborts iteration when a commit cannot be found"] pub struct Parents < 'commit , 'repo > { range : Range < usize > , commit : & 'commit Commit < 'repo > , }
    };
}

Parents!()