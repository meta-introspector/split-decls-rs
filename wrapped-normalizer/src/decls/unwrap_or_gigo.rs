macro_rules! unwrap_or_gigo {
    () => {
        # [doc = " If `opt` is `Some`, unwrap it. If `None`, panic if debug assertions"] # [doc = " are enabled and return `default` if debug assertions are not enabled."] # [doc = ""] # [doc = " Use this only if the only reason why `opt` could be `None` is bogus"] # [doc = " data from the provider."] # [inline (always)] fn unwrap_or_gigo < T > (opt : Option < T > , default : T) -> T { if let Some (val) = opt { val } else { debug_assert ! (false) ; default } }
    };
}

unwrap_or_gigo!();