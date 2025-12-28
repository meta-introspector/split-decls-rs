macro_rules! windows_console {
    () => {
        # [cfg (not (windows))] mod windows_console { # [inline] pub (crate) fn enable_ansi_colors () -> Option < bool > { None } }
    };
}

windows_console!();