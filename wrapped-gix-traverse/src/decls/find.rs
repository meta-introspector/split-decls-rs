macro_rules! deps {
    () => {
        Either!();
        Error!();
    };
}

macro_rules! find {
    () => {
        deps!();
        # [doc = " Find information about a commit by either getting it from a [`gix_commitgraph::Graph`], if"] # [doc = " present, or a [`gix_object::CommitRefIter`] otherwise."] pub fn find < 'cache , 'buf , Find > (cache : Option < & 'cache gix_commitgraph :: Graph > , objects : Find , id : & gix_hash :: oid , buf : & 'buf mut Vec < u8 > ,) -> Result < Either < 'buf , 'cache > , gix_object :: find :: existing_iter :: Error > where Find : gix_object :: Find , { match cache . and_then (| cache | cache . commit_by_id (id) . map (Either :: CachedCommit)) { Some (c) => Ok (c) , None => objects . find_commit_iter (id , buf) . map (Either :: CommitRefIter) , } }
    };
}

find!()