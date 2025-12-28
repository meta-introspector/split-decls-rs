macro_rules! deps {
    () => {
        RawStream!();
    };
}

macro_rules! choice {
    () => {
        deps!();
        # [cfg (feature = "auto")] fn choice (raw : & dyn RawStream) -> ColorChoice { let choice = ColorChoice :: global () ; match choice { ColorChoice :: Auto => { let clicolor = anstyle_query :: clicolor () ; let clicolor_enabled = clicolor . unwrap_or (false) ; let clicolor_disabled = ! clicolor . unwrap_or (true) ; if anstyle_query :: no_color () { ColorChoice :: Never } else if anstyle_query :: clicolor_force () { ColorChoice :: Always } else if clicolor_disabled { ColorChoice :: Never } else if raw . is_terminal () && (anstyle_query :: term_supports_color () || clicolor_enabled || anstyle_query :: is_ci ()) { ColorChoice :: Always } else { ColorChoice :: Never } } ColorChoice :: AlwaysAnsi | ColorChoice :: Always | ColorChoice :: Never => choice , } }
    };
}

choice!()