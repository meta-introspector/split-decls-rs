macro_rules! deps {
    () => {
        CompletionCandidate!();
        SubcommandCandidates!();
    };
}

macro_rules! complete_external_subcommand {
    () => {
        deps!();
        fn complete_external_subcommand (value : & str , completer : & SubcommandCandidates ,) -> Vec < CompletionCandidate > { debug ! ("complete_custom_arg_value: completer={completer:?}, value={value:?}") ; let mut values = Vec :: new () ; let custom_arg_values = completer . candidates () ; values . extend (custom_arg_values) ; values . retain (| comp | comp . get_value () . starts_with (value)) ; values }
    };
}

complete_external_subcommand!()