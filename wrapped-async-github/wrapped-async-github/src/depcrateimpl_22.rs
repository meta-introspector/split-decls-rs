// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl PullRequestListWidget { # [doc = " Start fetching the pull requests in the background."] # [doc = ""] # [doc = " This method spawns a background task that fetches the pull requests from the GitHub API."] # [doc = " The result of the fetch is then passed to the `on_load` or `on_err` methods."] fn run (& self) { let this = self . clone () ; tokio :: spawn (this . fetch_pulls ()) ; } async fn fetch_pulls (self) { self . set_loading_state (LoadingState :: Loading) ; match octocrab :: instance () . pulls ("ratatui" , "ratatui") . list () . sort (Sort :: Updated) . direction (Direction :: Descending) . send () . await { Ok (page) => self . on_load (& page) , Err (err) => self . on_err (& err) , } } fn on_load (& self , page : & Page < OctoPullRequest >) { let prs = page . items . iter () . map (Into :: into) ; let mut state = self . state . write () . unwrap () ; state . loading_state = LoadingState :: Loaded ; state . pull_requests . extend (prs) ; if ! state . pull_requests . is_empty () { state . table_state . select (Some (0)) ; } } fn on_err (& self , err : & octocrab :: Error) { self . set_loading_state (LoadingState :: Error (err . to_string ())) ; } fn set_loading_state (& self , state : LoadingState) { self . state . write () . unwrap () . loading_state = state ; } fn scroll_down (& self) { self . state . write () . unwrap () . table_state . scroll_down_by (1) ; } fn scroll_up (& self) { self . state . write () . unwrap () . table_state . scroll_up_by (1) ; } }
};
}
