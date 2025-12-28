macro_rules! deps {
    () => {
        Platform!();
        Error!();
    };
}

macro_rules! impl_865 {
    () => {
        deps!();
        # [doc = " Produce the iterator"] impl < 'repo > Platform < 'repo > { # [doc = " For each commit, let `filter` return `true` if it and its parents should be included in the traversal, or `false`"] # [doc = " if the traversal should exclude it and its ancestry entirely."] # [doc = ""] # [doc = " If `filter` is None, no pruning of the graph will be performed which is the default."] pub fn selected (self , mut filter : impl FnMut (& gix_hash :: oid) -> bool + 'repo ,) -> Result < revision :: Walk < 'repo > , Error > { let Platform { repo , tips , sorting , parents , use_commit_graph , commit_graph , mut boundary , hidden , } = self ; boundary . sort () ; Ok (revision :: Walk { repo , inner : Box :: new (gix_traverse :: commit :: Simple :: filtered (tips , & repo . objects , { let shallow_commits = repo . shallow_commits () ? ; let mut grafted_parents_to_skip = Vec :: new () ; let mut buf = Vec :: new () ; move | id | { if ! filter (id) { return false ; } let id = id . to_owned () ; if boundary . binary_search (& id) . is_ok () { return false ; } match shallow_commits . as_ref () { Some (commits) => { if let Ok (idx) = grafted_parents_to_skip . binary_search (& id) { grafted_parents_to_skip . remove (idx) ; return false ; } if commits . binary_search (& id) . is_ok () { if let Ok (commit) = repo . objects . find_commit_iter (& id , & mut buf) { grafted_parents_to_skip . extend (commit . parent_ids ()) ; grafted_parents_to_skip . sort () ; } } true } None => true , } } }) . sorting (sorting . into_simple () . expect ("for now there is nothing else")) ? . parents (parents) . hide (hidden) ? . commit_graph (commit_graph . or (use_commit_graph . map_or_else (| | self . repo . config . may_use_commit_graph () , Ok) ? . then (| | self . repo . commit_graph () . ok ()) . flatten ()) ,) . map (| res | res . map_err (iter :: Error :: from)) ,) , }) } # [doc = " Return an iterator to traverse all commits reachable as configured by the [Platform]."] # [doc = ""] # [doc = " # Performance"] # [doc = ""] # [doc = " It's highly recommended to set an [`object cache`](Repository::object_cache_size()) on the parent repo"] # [doc = " to greatly speed up performance if the returned id is supposed to be looked up right after."] pub fn all (self) -> Result < revision :: Walk < 'repo > , Error > { self . selected (| _ | true) } }
    };
}

impl_865!();