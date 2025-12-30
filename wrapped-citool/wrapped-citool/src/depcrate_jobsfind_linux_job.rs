// Generated macro for find_linux_job (function)
macro_rules! Depcrate_jobsfind_linux_job {
() => {
// Module: crate::jobs
// Provides: {"find_linux_job"}
// Dependencies: {}
pub fn find_linux_job < 'a > (jobs : & 'a [Job] , name : & str) -> anyhow :: Result < & 'a Job > { let Some (job) = jobs . iter () . find (| j | j . name == name) else { let available_jobs : Vec < & Job > = jobs . iter () . filter (| j | j . is_linux ()) . collect () ; let mut available_jobs = available_jobs . iter () . map (| j | j . name . to_string ()) . collect :: < Vec < _ > > () ; available_jobs . sort () ; return Err (anyhow :: anyhow ! ("Job {name} not found. The following jobs are available:\n{}" , available_jobs . join (", "))) ; } ; if ! job . is_linux () { return Err (anyhow :: anyhow ! ("Only Linux jobs can be executed locally")) ; } Ok (job) }
};
}
