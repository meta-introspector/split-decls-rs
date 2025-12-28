macro_rules! enable_ansi_colors {
    () => {
        # [doc = " Enable ANSI escape codes ([`ENABLE_VIRTUAL_TERMINAL_PROCESSING`](https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#output-sequences))"] # [doc = ""] # [doc = " For non-windows systems, returns `None`"] pub fn enable_ansi_colors () -> Option < bool > { windows_console :: enable_ansi_colors () }
    };
}

enable_ansi_colors!()