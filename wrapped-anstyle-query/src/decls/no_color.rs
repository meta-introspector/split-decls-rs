macro_rules! no_color {
    () => {
        # [doc = " Check [NO_COLOR] status"] # [doc = ""] # [doc = " When `true`, should prevent the addition of ANSI color."] # [doc = ""] # [doc = " User-level configuration files and per-instance command-line arguments should override"] # [doc = " [NO_COLOR]. A user should be able to export `$NO_COLOR` in their shell configuration file as a"] # [doc = " default, but configure a specific program in its configuration file to specifically enable"] # [doc = " color."] # [doc = ""] # [doc = " [NO_COLOR]: https://no-color.org/"] # [inline] pub fn no_color () -> bool { non_empty (std :: env :: var_os ("NO_COLOR") . as_deref ()) }
    };
}

no_color!()