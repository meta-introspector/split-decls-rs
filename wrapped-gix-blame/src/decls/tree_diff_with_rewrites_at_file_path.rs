macro_rules! deps {
    () => {
        TreeDiffChange!();
        Statistics!();
        Error!();
        Change!();
        Options!();
    };
}

macro_rules! tree_diff_with_rewrites_at_file_path {
    () => {
        deps!();
        # [allow (clippy :: too_many_arguments)] fn tree_diff_with_rewrites_at_file_path (odb : impl gix_object :: Find + gix_object :: FindHeader , file_path : & BStr , stats : & mut Statistics , state : & mut gix_diff :: tree :: State , resource_cache : & mut gix_diff :: blob :: Platform , parent_tree_iter : gix_object :: TreeRefIter < '_ > , tree_iter : gix_object :: TreeRefIter < '_ > , rewrites : gix_diff :: Rewrites ,) -> Result < Option < TreeDiffChange > , Error > { let mut change : Option < gix_diff :: tree_with_rewrites :: Change > = None ; let options : gix_diff :: tree_with_rewrites :: Options = gix_diff :: tree_with_rewrites :: Options { location : Some (gix_diff :: tree :: recorder :: Location :: Path) , rewrites : Some (rewrites) , } ; let result = gix_diff :: tree_with_rewrites (parent_tree_iter , tree_iter , resource_cache , state , & odb , | change_ref | -> Result < _ , std :: convert :: Infallible > { if change_ref . location () == file_path { change = Some (change_ref . into_owned ()) ; Ok (gix_diff :: tree_with_rewrites :: Action :: Cancel) } else { Ok (gix_diff :: tree_with_rewrites :: Action :: Continue) } } , options ,) ; stats . trees_diffed_with_rewrites += 1 ; match result { Ok (_) | Err (gix_diff :: tree_with_rewrites :: Error :: Diff (gix_diff :: tree :: Error :: Cancelled)) => { Ok (change . map (Into :: into)) } Err (error) => Err (Error :: DiffTreeWithRewrites (error)) , } }
    };
}

tree_diff_with_rewrites_at_file_path!()