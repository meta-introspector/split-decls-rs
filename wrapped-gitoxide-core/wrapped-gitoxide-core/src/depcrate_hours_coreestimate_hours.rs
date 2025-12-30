// Generated macro for estimate_hours (function)
macro_rules! Depcrate_hours_coreestimate_hours {
() => {
// Module: crate::hours::core
// Provides: {"estimate_hours"}
// Dependencies: {}
pub fn estimate_hours (commits : & [(u32 , super :: SignatureRef < 'static >)] , stats : & [(u32 , FileStats , LineStats)] ,) -> WorkByEmail { assert ! (! commits . is_empty ()) ; const MAX_COMMIT_DIFFERENCE_IN_MINUTES : f32 = 2.0 * MINUTES_PER_HOUR ; const FIRST_COMMIT_ADDITION_IN_MINUTES : f32 = 2.0 * MINUTES_PER_HOUR ; let hours_for_commits = { let mut hours = 0.0 ; let mut commits = commits . iter () . map (| t | & t . 1) . rev () ; let mut cur = commits . next () . expect ("at least one commit if we are here") ; for next in commits { let change_in_minutes = (next . seconds () . saturating_sub (cur . seconds ())) as f32 / MINUTES_PER_HOUR ; if change_in_minutes < MAX_COMMIT_DIFFERENCE_IN_MINUTES { hours += change_in_minutes / MINUTES_PER_HOUR ; } else { hours += FIRST_COMMIT_ADDITION_IN_MINUTES / MINUTES_PER_HOUR ; } cur = next ; } hours } ; let author = & commits [0] . 1 ; let (files , lines) = if ! stats . is_empty () { { commits . iter () . map (| t | & t . 0) . fold ((FileStats :: default () , LineStats :: default ()) , | mut acc , id | match stats . binary_search_by (| t | t . 0 . cmp (id)) { Ok (idx) => { let t = & stats [idx] ; acc . 0 . add (& t . 1) ; acc . 1 . add (& t . 2) ; acc } Err (_) => acc , }) } } else { Default :: default () } ; WorkByEmail { name : author . name , email : author . email , hours : FIRST_COMMIT_ADDITION_IN_MINUTES / 60.0 + hours_for_commits , num_commits : commits . len () as u32 , files , lines , } }
};
}
