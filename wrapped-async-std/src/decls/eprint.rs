macro_rules! eprint {
    () => {
        # [doc = " Prints to the standard error."] # [doc = ""] # [doc = " Equivalent to the [`print!`] macro, except that output goes to"] # [doc = " [`io::stderr`] instead of `io::stdout`. See [`print!`] for"] # [doc = " example usage."] # [doc = ""] # [doc = " Use `eprint!` only for error and progress messages. Use `print!`"] # [doc = " instead for the primary output of your program."] # [doc = ""] # [doc = " [`io::stderr`]: io/struct.Stderr.html"] # [doc = " [`print!`]: macro.print.html"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if writing to `io::stderr` fails."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # async_std::task::block_on(async {"] # [doc = " #"] # [doc = " use async_std::eprint;"] # [doc = ""] # [doc = " eprint!(\"Error: Could not complete task\").await;"] # [doc = " #"] # [doc = " # })"] # [doc = " ```"] # [cfg (feature = "unstable")] # [cfg_attr (feature = "docs" , doc (cfg (unstable)))] # [macro_export] macro_rules ! eprint { ($ ($ arg : tt) *) => ($ crate :: io :: _eprint (format_args ! ($ ($ arg) *))) }
    };
}

eprint!()