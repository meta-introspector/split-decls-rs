// Generated macro for impl_40 (impl)
macro_rules! Depcrate_corpus_engineimpl_40 {
() => {
// Module: crate::corpus::engine
// Provides: {"impl_40"}
// Dependencies: {}
impl Engine { # [doc = " Open the corpus DB or create it."] pub fn open_or_create (db : PathBuf , state : State) -> anyhow :: Result < Engine > { let con = crate :: corpus :: db :: create (db) . context ("Could not open or create database") ? ; Ok (Engine { con , state }) } # [doc = " Run on the existing set of repositories we have already seen or obtain them from `path` if there is none yet."] pub fn run (& mut self , corpus_path : PathBuf , threads : Option < usize > , dry_run : bool , repo_sql_suffix : Option < String > , allowed_task_names : Vec < String > ,) -> anyhow :: Result < () > { let tasks = self . tasks_or_insert (& allowed_task_names) ? ; if tasks . is_empty () { bail ! ("Cannot run without any task to perform on the repositories") ; } let (corpus_path , corpus_id) = self . prepare_corpus_path (corpus_path) ? ; let gitoxide_id = self . gitoxide_version_id_or_insert () ? ; let runner_id = self . runner_id_or_insert () ? ; let repos = self . find_repos_or_insert (& corpus_path , corpus_id , repo_sql_suffix) ? ; self . perform_run (& corpus_path , gitoxide_id , runner_id , & tasks , repos , threads , dry_run) } pub fn refresh (& mut self , corpus_path : PathBuf) -> anyhow :: Result < () > { let (corpus_path , corpus_id) = self . prepare_corpus_path (corpus_path) ? ; let repos = self . refresh_repos (& corpus_path , corpus_id) ? ; self . state . progress . set_name ("refresh repos" . into ()) ; self . state . progress . info (format ! ("Added or updated {} repositories under '{corpus_path}'" , repos . len () , corpus_path = corpus_path . display () ,)) ; Ok (()) } }
};
}
