macro_rules! onbranch_matches {
    () => {
        fn onbranch_matches (condition : & BStr , conditional :: Context { branch_name , .. } : conditional :: Context < '_ > ,) -> Option < () > { let branch_name = branch_name ? ; let (_ , branch_name) = branch_name . category_and_short_name () . filter (| (cat , _) | * cat == Category :: LocalBranch) ? ; let condition = if condition . ends_with (b"/") { let mut condition : BString = condition . into () ; condition . push_str ("**") ; Cow :: Owned (condition) } else { condition . into () } ; gix_glob :: wildmatch (condition . as_ref () , branch_name , gix_glob :: wildmatch :: Mode :: NO_MATCH_SLASH_LITERAL ,) . then_some (()) }
    };
}

onbranch_matches!();