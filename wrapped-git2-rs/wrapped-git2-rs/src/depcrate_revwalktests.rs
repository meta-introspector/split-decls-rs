// Generated macro for tests (module)
macro_rules! Depcrate_revwalktests {
() => {
// Module: crate::revwalk
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] fn smoke () { let (_td , repo) = crate :: test :: repo_init () ; let head = repo . head () . unwrap () ; let target = head . target () . unwrap () ; let mut walk = repo . revwalk () . unwrap () ; walk . push (target) . unwrap () ; let oids : Vec < crate :: Oid > = walk . by_ref () . collect :: < Result < Vec < _ > , _ > > () . unwrap () ; assert_eq ! (oids . len () , 1) ; assert_eq ! (oids [0] , target) ; walk . reset () . unwrap () ; walk . push_head () . unwrap () ; assert_eq ! (walk . by_ref () . count () , 1) ; walk . reset () . unwrap () ; walk . push_head () . unwrap () ; walk . hide_head () . unwrap () ; assert_eq ! (walk . by_ref () . count () , 0) ; } # [test] fn smoke_hide_cb () { let (_td , repo) = crate :: test :: repo_init () ; let head = repo . head () . unwrap () ; let target = head . target () . unwrap () ; let mut walk = repo . revwalk () . unwrap () ; walk . push (target) . unwrap () ; let oids : Vec < crate :: Oid > = walk . by_ref () . collect :: < Result < Vec < _ > , _ > > () . unwrap () ; assert_eq ! (oids . len () , 1) ; assert_eq ! (oids [0] , target) ; walk . reset () . unwrap () ; walk . push_head () . unwrap () ; assert_eq ! (walk . by_ref () . count () , 1) ; walk . reset () . unwrap () ; walk . push_head () . unwrap () ; let mut hide_cb = | oid | oid == target ; let mut walk = walk . with_hide_callback (& mut hide_cb) . unwrap () ; assert_eq ! (walk . by_ref () . count () , 0) ; let mut walk = walk . into_inner () . unwrap () ; walk . push_head () . unwrap () ; assert_eq ! (walk . by_ref () . count () , 1) ; } }
};
}
