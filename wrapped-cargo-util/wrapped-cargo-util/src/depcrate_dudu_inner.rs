// Generated macro for du_inner (function)
macro_rules! Depcrate_dudu_inner {
() => {
// Module: crate::du
// Provides: {"du_inner"}
// Dependencies: {}
fn du_inner (path : & Path , patterns : & [& str]) -> Result < u64 > { let mut builder = OverrideBuilder :: new (path) ; for pattern in patterns { builder . add (pattern) ? ; } let overrides = builder . build () ? ; let mut builder = WalkBuilder :: new (path) ; builder . overrides (overrides) . hidden (false) . parents (false) . ignore (false) . git_global (false) . git_ignore (false) . git_exclude (false) ; let walker = builder . build_parallel () ; let total = Arc :: new (Mutex :: new (0u64)) ; let err = Arc :: new (Mutex :: new (None)) ; walker . run (| | { Box :: new (| entry | { match entry { Ok (entry) => match entry . metadata () { Ok (meta) => { if meta . is_file () { let mut lock = total . lock () . unwrap () ; * lock += meta . len () ; } } Err (e) => { * err . lock () . unwrap () = Some (e . into ()) ; return WalkState :: Quit ; } } , Err (e) => { * err . lock () . unwrap () = Some (e . into ()) ; return WalkState :: Quit ; } } WalkState :: Continue }) }) ; if let Some (e) = err . lock () . unwrap () . take () { return Err (e) ; } let total = * total . lock () . unwrap () ; Ok (total) }
};
}
