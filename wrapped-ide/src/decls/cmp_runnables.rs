macro_rules! deps {
    () => {
        Runnable!();
    };
}

macro_rules! cmp_runnables {
    () => {
        deps!();
        fn cmp_runnables (Runnable { nav , kind , .. } : & Runnable , Runnable { nav : nav_b , kind : kind_b , .. } : & Runnable ,) -> std :: cmp :: Ordering { nav . full_range . start () . cmp (& nav_b . full_range . start ()) . then_with (| | { let t_0 = | | TextSize :: from (0) ; nav . focus_range . map_or_else (t_0 , | it | it . start ()) . cmp (& nav_b . focus_range . map_or_else (t_0 , | it | it . start ())) }) . then_with (| | kind . disc () . cmp (& kind_b . disc ())) . then_with (| | nav . name . as_str () . cmp (nav_b . name . as_str ())) }
    };
}

cmp_runnables!()