// Generated macro for JOBS_YML_PATH (const)
macro_rules! DepcrateJOBS_YML_PATH {
() => {
// Module: crate
// Provides: {"JOBS_YML_PATH"}
// Dependencies: {}
const JOBS_YML_PATH : & str = concat ! (env ! ("CARGO_MANIFEST_DIR") , "/../github-actions/jobs.yml") ;
};
}
