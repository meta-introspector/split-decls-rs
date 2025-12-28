macro_rules! deps {
    () => {
        Either!();
        ParentIds!();
        Error!();
    };
}

macro_rules! collect_parents {
    () => {
        deps!();
        fn collect_parents (commit : gix_traverse :: commit :: Either < '_ , '_ > , odb : & impl gix_object :: Find , cache : Option < & gix_commitgraph :: Graph > , buf : & mut Vec < u8 > ,) -> Result < ParentIds , Error > { let mut parent_ids : ParentIds = Default :: default () ; match commit { gix_traverse :: commit :: Either :: CachedCommit (commit) => { let cache = cache . as_ref () . expect ("find returned a cached commit, so we expect cache to be present") ; for parent_pos in commit . iter_parents () { let parent = cache . commit_at (parent_pos ?) ; parent_ids . push ((parent . id () . to_owned () , parent . committer_timestamp () as i64)) ; } } gix_traverse :: commit :: Either :: CommitRefIter (commit_ref_iter) => { for id in commit_ref_iter . parent_ids () { let parent = odb . find_commit_iter (id . as_ref () , buf) . ok () ; let parent_commit_time = parent . and_then (| parent | parent . committer () . ok () . map (| committer | committer . seconds ())) . unwrap_or_default () ; parent_ids . push ((id , parent_commit_time)) ; } } } Ok (parent_ids) }
    };
}

collect_parents!();