macro_rules! deps {
    () => {
        ArgValueCandidates!();
        CompletionCandidate!();
    };
}

macro_rules! complete_custom_arg_value {
    () => {
        deps!();
        fn complete_custom_arg_value (value : & OsStr , completer : & ArgValueCandidates ,) -> Vec < CompletionCandidate > { debug ! ("complete_custom_arg_value: completer={completer:?}, value={value:?}") ; let mut values = completer . candidates () ; values . retain (| comp | comp . get_value () . starts_with (& value . to_string_lossy ())) ; values }
    };
}

complete_custom_arg_value!()