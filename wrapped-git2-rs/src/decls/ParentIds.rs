macro_rules! deps {
    () => {
        Commit!();
    };
}

macro_rules! ParentIds {
    () => {
        deps!();
        # [doc = " An iterator over the parent commits' ids of a commit."] # [doc = ""] # [doc = " Aborts iteration when a commit cannot be found"] pub struct ParentIds < 'commit > { range : Range < usize > , commit : & 'commit Commit < 'commit > , }
    };
}

ParentIds!()