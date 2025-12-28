macro_rules! deps {
    () => {
        MatchFailed!();
    };
}

macro_rules! match_error {
    () => {
        deps!();
        macro_rules ! match_error { ($ e : expr) => { { MatchFailed { reason : if recording_match_fail_reasons () { Some (format ! ("{}" , $ e)) } else { None } } } } ; ($ fmt : expr , $ ($ arg : tt) +) => { { MatchFailed { reason : if recording_match_fail_reasons () { Some (format ! ($ fmt , $ ($ arg) +)) } else { None } } } } ; }
    };
}

match_error!();