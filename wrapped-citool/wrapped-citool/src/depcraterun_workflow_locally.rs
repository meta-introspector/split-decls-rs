// Generated macro for run_workflow_locally (function)
macro_rules! Depcraterun_workflow_locally {
() => {
// Module: crate
// Provides: {"run_workflow_locally"}
// Dependencies: {}
fn run_workflow_locally (db : JobDatabase , job_type : JobType , name : String) -> anyhow :: Result < () > { let jobs = match job_type { JobType :: Auto => & db . auto_jobs , JobType :: PR => & db . pr_jobs , } ; let job = jobs :: find_linux_job (jobs , & name) . with_context (| | format ! ("Cannot find job {name}")) ? ; let mut custom_env : BTreeMap < String , String > = BTreeMap :: new () ; if name . starts_with ("dist-") { if name . ends_with ("-alt") { custom_env . insert ("DEPLOY_ALT" . to_string () , "1" . to_string ()) ; } else { custom_env . insert ("DEPLOY" . to_string () , "1" . to_string ()) ; } } custom_env . extend (job . env . iter () . map (| (key , value) | { let value = match value { Value :: Bool (value) => value . to_string () , Value :: Number (value) => value . to_string () , Value :: String (value) => value . clone () , _ => panic ! ("Unexpected type for environment variable {key} Only bool/number/string is supported.") } ; (key . clone () , value) })) ; let mut cmd = Command :: new (Path :: new (DOCKER_DIRECTORY) . join ("run.sh")) ; cmd . arg (job . image ()) ; cmd . envs (custom_env) ; eprintln ! ("Executing {cmd:?}") ; let result = cmd . spawn () ? . wait () ? ; if ! result . success () { Err (anyhow :: anyhow ! ("Job failed")) } else { Ok (()) } }
};
}
