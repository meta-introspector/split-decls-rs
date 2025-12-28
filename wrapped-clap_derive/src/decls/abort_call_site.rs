macro_rules! abort_call_site {
    () => {
        macro_rules ! abort_call_site { ($ ($ format : tt) +) => { { let span = proc_macro2 :: Span :: call_site () ; abort ! (span , $ ($ format) +) } } ; }
    };
}

abort_call_site!();