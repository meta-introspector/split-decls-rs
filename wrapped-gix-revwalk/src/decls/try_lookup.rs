macro_rules! deps {
    () => {
        Either!();
        Graph!();
        LazyCommit!();
    };
}

macro_rules! try_lookup {
    () => {
        deps!();
        fn try_lookup < 'graph , 'cache > (id : & gix_hash :: oid , objects : & dyn gix_object :: Find , cache : Option < & 'cache gix_commitgraph :: Graph > , buf : & 'graph mut Vec < u8 > ,) -> Result < Option < LazyCommit < 'graph , 'cache > > , gix_object :: find :: existing_iter :: Error > { if let Some (cache) = cache { if let Some (pos) = cache . lookup (id) { return Ok (Some (LazyCommit { backing : Either :: Right ((cache , pos)) , })) ; } } # [allow (clippy :: manual_map)] Ok (match objects . try_find (id , buf) . map_err (gix_object :: find :: existing_iter :: Error :: Find) ? { Some (data) => data . kind . is_commit () . then_some (LazyCommit { backing : Either :: Left (buf) , }) , None => None , } ,) }
    };
}

try_lookup!()