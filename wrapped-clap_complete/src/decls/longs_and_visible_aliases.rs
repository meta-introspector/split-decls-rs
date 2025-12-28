macro_rules! deps {
    () => {
        CompletionCandidate!();
    };
}

macro_rules! longs_and_visible_aliases {
    () => {
        deps!();
        # [doc = " Gets all the long options, their visible aliases and flags of a [`clap::Command`] with formatted `--` prefix."] # [doc = " Includes `help` and `version` depending on the [`clap::Command`] settings."] fn longs_and_visible_aliases (p : & clap :: Command) -> Vec < CompletionCandidate > { debug ! ("longs: name={}" , p . get_name ()) ; p . get_arguments () . filter_map (| a | { a . get_long_and_visible_aliases () . map (| longs | { longs . into_iter () . map (| s | populate_arg_candidate (CompletionCandidate :: new (format ! ("--{s}")) , a)) }) }) . flatten () . collect () }
    };
}

longs_and_visible_aliases!()