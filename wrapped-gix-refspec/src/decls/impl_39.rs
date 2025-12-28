macro_rules! deps {
    () => {
        Fetch!();
        Item!();
        Push!();
        MatchGroup!();
        Operation!();
        RefSpecRef!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        # [doc = " Initialization"] impl < 'a > MatchGroup < 'a > { # [doc = " Take all the fetch ref specs from `specs` get a match group ready."] pub fn from_fetch_specs (specs : impl IntoIterator < Item = RefSpecRef < 'a > >) -> Self { MatchGroup { specs : specs . into_iter () . filter (| s | s . op == Operation :: Fetch) . collect () , } } # [doc = " Take all the push ref specs from `specs` get a match group ready."] pub fn from_push_specs (specs : impl IntoIterator < Item = RefSpecRef < 'a > >) -> Self { MatchGroup { specs : specs . into_iter () . filter (| s | s . op == Operation :: Push) . collect () , } } }
    };
}

impl_39!()