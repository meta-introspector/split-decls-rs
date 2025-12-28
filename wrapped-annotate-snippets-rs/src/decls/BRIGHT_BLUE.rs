macro_rules! BRIGHT_BLUE {
    () => {
        const BRIGHT_BLUE : Style = if USE_WINDOWS_COLORS { AnsiColor :: BrightCyan . on_default () } else { AnsiColor :: BrightBlue . on_default () } ;
    };
}

BRIGHT_BLUE!()