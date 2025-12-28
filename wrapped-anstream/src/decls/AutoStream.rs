macro_rules! deps {
    () => {
        RawStream!();
        StreamInner!();
    };
}

macro_rules! AutoStream {
    () => {
        deps!();
        # [doc = " [`std::io::Write`] that adapts ANSI escape codes to the underlying `Write`s capabilities"] # [doc = ""] # [doc = " This includes"] # [doc = " - Stripping colors for non-terminals"] # [doc = " - Respecting env variables like [NO_COLOR](https://no-color.org/) or [CLICOLOR](https://bixense.com/clicolors/)"] # [doc = " - *(windows)* Falling back to the wincon API where [ENABLE_VIRTUAL_TERMINAL_PROCESSING](https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#output-sequences) is unsupported"] # [doc = ""] # [doc = " You can customize auto-detection by calling into"] # [doc = " [anstyle_query](https://docs.rs/anstyle-query/latest/anstyle_query/)"] # [doc = " to get a [`ColorChoice`] and then calling [`AutoStream::new(stream, choice)`]."] # [derive (Debug)] pub struct AutoStream < S : RawStream > { inner : StreamInner < S > , }
    };
}

AutoStream!();