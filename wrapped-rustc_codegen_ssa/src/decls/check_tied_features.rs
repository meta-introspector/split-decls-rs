macro_rules! check_tied_features {
    () => {
        # [doc = " Given a map from target_features to whether they are enabled or disabled, ensure only valid"] # [doc = " combinations are allowed."] pub fn check_tied_features (sess : & Session , features : & FxHashMap < & str , bool > ,) -> Option < & 'static [& 'static str] > { if ! features . is_empty () { for tied in sess . target . tied_target_features () { let mut tied_iter = tied . iter () ; let enabled = features . get (tied_iter . next () . unwrap ()) ; if tied_iter . any (| f | enabled != features . get (f)) { return Some (tied) ; } } } None }
    };
}

check_tied_features!();