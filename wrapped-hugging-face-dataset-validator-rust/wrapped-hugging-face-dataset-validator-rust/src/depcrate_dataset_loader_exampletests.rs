// Generated macro for tests (module)
macro_rules! Depcrate_dataset_loader_exampletests {
() => {
// Module: crate::dataset_loader_example
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_dataset_loader () { let dataset_dir = "solfunmeme-hf-dataset" ; if ! Path :: new (dataset_dir) . exists () { println ! ("Skipping test - dataset not found at {}" , dataset_dir) ; return ; } let loader = DatasetLoader :: new (dataset_dir) . unwrap () ; let splits = loader . get_splits () ; assert ! (! splits . is_empty ()) ; assert ! (splits . contains (& "train" . to_string ())) ; let stats = loader . get_stats () . unwrap () ; assert ! (stats . get ("train") . unwrap () > & 0) ; } }
};
}
