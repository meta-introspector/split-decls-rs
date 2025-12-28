macro_rules! deps {
    () => {
        CompletionCandidate!();
    };
}

macro_rules! subcommands {
    () => {
        deps!();
        # [doc = " Gets subcommands of [`clap::Command`] in the form of `(\"name\", \"bin_name\")`."] # [doc = ""] # [doc = " Subcommand `rustup toolchain install` would be converted to"] # [doc = " `(\"install\", \"rustup toolchain install\")`."] fn subcommands (p : & clap :: Command) -> Vec < CompletionCandidate > { debug ! ("subcommands: name={}" , p . get_name ()) ; debug ! ("subcommands: Has subcommands...{:?}" , p . has_subcommands ()) ; p . get_subcommands () . flat_map (| sc | { sc . get_name_and_visible_aliases () . into_iter () . map (| s | populate_command_candidate (CompletionCandidate :: new (s . to_string ()) , p , sc)) . chain (sc . get_aliases () . map (| s | { populate_command_candidate (CompletionCandidate :: new (s . to_string ()) , p , sc) . hide (true) })) }) . collect () }
    };
}

subcommands!();