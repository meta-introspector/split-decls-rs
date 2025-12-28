macro_rules! deps {
    () => {
        SubcommandCandidates!();
        CompletionCandidate!();
    };
}

macro_rules! complete_subcommand {
    () => {
        deps!();
        fn complete_subcommand (value : & str , cmd : & clap :: Command) -> Vec < CompletionCandidate > { debug ! ("complete_subcommand: cmd={:?}, value={:?}" , cmd . get_name () , value) ; let mut scs : Vec < CompletionCandidate > = subcommands (cmd) . into_iter () . filter (| x | x . get_value () . starts_with (value)) . collect () ; if cmd . is_allow_external_subcommands_set () { let external_completer = cmd . get :: < SubcommandCandidates > () ; if let Some (completer) = external_completer { scs . extend (complete_external_subcommand (value , completer)) ; } } scs . sort () ; scs . dedup () ; scs }
    };
}

complete_subcommand!()