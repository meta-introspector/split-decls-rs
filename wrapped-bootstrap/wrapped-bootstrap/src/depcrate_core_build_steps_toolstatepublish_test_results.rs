// Generated macro for publish_test_results (function)
macro_rules! Depcrate_core_build_steps_toolstatepublish_test_results {
() => {
// Module: crate::core::build_steps::toolstate
// Provides: {"publish_test_results"}
// Dependencies: {}
# [doc = " Updates the \"history\" files with the latest results."] # [doc = ""] # [doc = " These results will later be promoted to `latest.json` by the"] # [doc = " `publish_toolstate.py` script if the PR passes all tests and is merged to"] # [doc = " master."] fn publish_test_results (builder : & Builder < '_ > , current_toolstate : & ToolstateData) { let commit = helpers :: git (None) . arg ("rev-parse") . arg ("HEAD") . run_capture (builder) . stdout () ; let toolstate_serialized = t ! (serde_json :: to_string (& current_toolstate)) ; let history_path = Path :: new (TOOLSTATE_DIR) . join ("history") . join (format ! ("{}.tsv" , OS . expect ("linux/windows only"))) ; let mut file = t ! (fs :: read_to_string (& history_path)) ; let end_of_first_line = file . find ('\n') . unwrap () ; file . insert_str (end_of_first_line , & format ! ("\n{}\t{}" , commit . trim () , toolstate_serialized)) ; t ! (fs :: write (& history_path , file)) ; }
};
}
