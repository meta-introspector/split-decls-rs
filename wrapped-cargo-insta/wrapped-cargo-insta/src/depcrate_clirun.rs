// Generated macro for run (function)
macro_rules! Depcrate_clirun {
() => {
// Module: crate::cli
// Provides: {"run"}
// Dependencies: {}
pub (crate) fn run () -> Result < () , Box < dyn Error > > { let mut args : Vec < _ > = env :: args_os () . collect () ; if env :: var ("CARGO") . is_ok () && args . get (1) . and_then (| x | x . to_str ()) == Some ("insta") { args . remove (1) ; } let opts = Opts :: parse_from (args) ; handle_color (opts . color) ; match opts . command { Command :: Review (ref cmd) | Command :: Accept (ref cmd) | Command :: Reject (ref cmd) => { review_snapshots (cmd . quiet , cmd . snapshot_filter . as_deref () , & handle_target_args (& cmd . target_args , & []) ? , match opts . command { Command :: Review (_) => None , Command :: Accept (_) => Some (Operation :: Accept) , Command :: Reject (_) => Some (Operation :: Reject) , _ => unreachable ! () , } ,) } Command :: Test (cmd) => test_run (cmd , opts . color . unwrap_or (ColorWhen :: Auto)) , Command :: Show (cmd) => show_cmd (cmd) , Command :: PendingSnapshots (cmd) => pending_snapshots_cmd (cmd) , } }
};
}
