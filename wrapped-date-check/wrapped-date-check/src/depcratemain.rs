// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let mut args = env :: args () ; if args . len () == 1 { eprintln ! ("error: expected root of Markdown directory as CLI argument") ; process :: exit (1) ; } let root_dir = args . nth (1) . unwrap () ; let root_dir_path = Path :: new (& root_dir) ; let glob_pat = format ! ("{}/**/*.md" , root_dir) ; let today_chrono = Utc :: now () . date_naive () ; let current_month = Date { year : today_chrono . year_ce () . 1 , month : today_chrono . month () } ; let dates_by_file = collect_dates (glob (& glob_pat) . unwrap () . map (Result :: unwrap)) ; let dates_by_file : BTreeMap < _ , _ > = filter_dates (current_month , 6 , dates_by_file . into_iter ()) . collect () ; if dates_by_file . is_empty () { println ! ("empty") ; } else { println ! ("Date Reference Triage for {}" , current_month) ; println ! ("## Procedure") ; println ! () ; println ! ("Each of these dates should be checked to see if the docs they annotate are \
             up-to-date. Each date should be updated (in the Markdown file where it appears) to \
             use the current month ({current_month}), or removed if the docs it annotates are not \
             expected to fall out of date quickly." , current_month = today_chrono . format ("%B %Y") ,) ; println ! () ; println ! ("Please check off each date once a PR to update it (and, if applicable, its \
             surrounding docs) has been merged. Please also mention that you are working on a \
             particular set of dates so duplicate work is avoided.") ; println ! () ; println ! ("Finally, once all the dates have been updated, please close this issue.") ; println ! () ; println ! ("## Dates") ; println ! () ; for (path , dates) in dates_by_file { println ! ("- {}" , path . strip_prefix (& root_dir_path) . unwrap_or (& path) . display () ,) ; for (line , date) in dates { println ! ("  - [ ] line {}: {}" , line , date) ; } } println ! () ; } }
};
}
