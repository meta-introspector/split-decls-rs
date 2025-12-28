macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! is_terminal {
    () => {
        deps!();
        # [doc = " Returns `true` if `this` is a terminal."] # [doc = ""] # [doc = " This is equivalent to calling `this.is_terminal()` and exists only as a"] # [doc = " convenience to calling the trait method [`IsTerminal::is_terminal`]"] # [doc = " without importing the trait."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " if is_terminal::is_terminal(&std::io::stdout()) {"] # [doc = "     println!(\"stdout is a terminal\")"] # [doc = " }"] # [doc = " ```"] pub fn is_terminal < T : IsTerminal > (this : T) -> bool { this . is_terminal () }
    };
}

is_terminal!();