macro_rules! deps {
    () => {
        CompletionCandidate!();
    };
}

macro_rules! hidden_longs_aliases {
    () => {
        deps!();
        # [doc = " Gets all the long hidden aliases and flags of a [`clap::Command`]."] fn hidden_longs_aliases (p : & clap :: Command) -> Vec < CompletionCandidate > { debug ! ("longs: name={}" , p . get_name ()) ; p . get_arguments () . filter_map (| a | { a . get_aliases () . map (| longs | { longs . into_iter () . map (| s | { populate_arg_candidate (CompletionCandidate :: new (format ! ("--{s}")) , a) . hide (true) }) }) }) . flatten () . collect () }
    };
}

hidden_longs_aliases!();