macro_rules! deps {
    () => {
        MatchCollector!();
        SsrMatches!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl From < MatchCollector > for SsrMatches { fn from (mut match_collector : MatchCollector) -> Self { let mut matches = SsrMatches :: default () ; for (_ , m) in match_collector . matches_by_node . drain () { matches . matches . push (m) ; } matches . matches . sort_by (| a , b | { a . range . file_id . cmp (& b . range . file_id) . then_with (| | a . range . range . start () . cmp (& b . range . range . start ())) }) ; matches } }
    };
}

impl_37!()