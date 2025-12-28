macro_rules! deps {
    () => {
        CompletionContext!();
    };
}

macro_rules! compute_exact_name_match {
    () => {
        deps!();
        fn compute_exact_name_match (ctx : & CompletionContext < '_ > , completion_name : & str) -> bool { ctx . expected_name . as_ref () . is_some_and (| name | name . text () == completion_name) }
    };
}

compute_exact_name_match!()