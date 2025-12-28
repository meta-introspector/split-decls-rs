macro_rules! trace {
    () => {
        # [doc = " Calls `log::trace!` only if the `trace` cargo feature is enabled."] macro_rules ! trace { ($ ($ arg : tt) +) => (# [cfg (feature = "trace")] log :: trace ! ($ ($ arg) +)) }
    };
}

trace!();