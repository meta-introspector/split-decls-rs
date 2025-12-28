macro_rules! deps {
    () => {
        NavigationTarget!();
    };
}

macro_rules! nav_for_branch_exit_points {
    () => {
        deps!();
        fn nav_for_branch_exit_points (sema : & Semantics < '_ , RootDatabase > , token : & SyntaxToken ,) -> Option < Vec < NavigationTarget > > { let db = sema . db ; let navs = match token . kind () { T ! [match] => find_branch_root (sema , token) . into_iter () . filter_map (| node | { let file_id = sema . hir_file_for (& node) ; let match_expr = ast :: MatchExpr :: cast (node) ? ; let focus_range = match_expr . match_token () ? . text_range () ; let match_expr_in_file = InFile :: new (file_id , match_expr . into ()) ; Some (expr_to_nav (db , match_expr_in_file , Some (focus_range))) }) . flatten () . collect_vec () , T ! [=>] => find_branch_root (sema , token) . into_iter () . filter_map (| node | { let match_arm = ast :: MatchArm :: cast (node) ? ; let match_expr = sema . ancestors_with_macros (match_arm . syntax () . clone ()) . find_map (ast :: MatchExpr :: cast) ? ; let file_id = sema . hir_file_for (match_expr . syntax ()) ; let focus_range = match_arm . fat_arrow_token () ? . text_range () ; let match_expr_in_file = InFile :: new (file_id , match_expr . into ()) ; Some (expr_to_nav (db , match_expr_in_file , Some (focus_range))) }) . flatten () . collect_vec () , T ! [if] => find_branch_root (sema , token) . into_iter () . filter_map (| node | { let file_id = sema . hir_file_for (& node) ; let if_expr = ast :: IfExpr :: cast (node) ? ; let focus_range = if_expr . if_token () ? . text_range () ; let if_expr_in_file = InFile :: new (file_id , if_expr . into ()) ; Some (expr_to_nav (db , if_expr_in_file , Some (focus_range))) }) . flatten () . collect_vec () , _ => return Some (Vec :: new ()) , } ; Some (navs) }
    };
}

nav_for_branch_exit_points!();