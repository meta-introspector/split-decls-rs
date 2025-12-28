macro_rules! Expander {
    () => {
        # [doc = " Expands argfiles in command line arguments."] # [derive (Default)] struct Expander { shell_argfiles : bool , next_is_unstable_option : bool , expanded : Vec < String > , }
    };
}

Expander!();