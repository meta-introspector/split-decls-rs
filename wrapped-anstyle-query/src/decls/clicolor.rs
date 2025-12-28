macro_rules! clicolor {
    () => {
        # [doc = " Check [CLICOLOR] status"] # [doc = ""] # [doc = " - When `true`, ANSI colors are supported and should be used when the program isn't piped,"] # [doc = "   similar to [`term_supports_color`]"] # [doc = " - When `false`, don’t output ANSI color escape codes, similar to [`no_color`]"] # [doc = ""] # [doc = " See also:"] # [doc = " - [terminfo](https://crates.io/crates/terminfo) or [term](https://crates.io/crates/term) for"] # [doc = "   checking termcaps"] # [doc = " - [termbg](https://crates.io/crates/termbg) for detecting background color"] # [doc = ""] # [doc = " [CLICOLOR]: https://bixense.com/clicolors/"] # [inline] pub fn clicolor () -> Option < bool > { let value = std :: env :: var_os ("CLICOLOR") ? ; Some (value != "0") }
    };
}

clicolor!();