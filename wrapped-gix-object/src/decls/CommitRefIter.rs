macro_rules! deps {
    () => {
        CommitRef!();
    };
}

macro_rules! CommitRefIter {
    () => {
        deps!();
        # [doc = " Like [`CommitRef`], but as `Iterator` to support (up to) entirely allocation free parsing."] # [doc = " It's particularly useful to traverse the commit graph without ever allocating arrays for parents."] # [derive (Copy , Clone)] pub struct CommitRefIter < 'a > { data : & 'a [u8] , state : commit :: ref_iter :: State , }
    };
}

CommitRefIter!()