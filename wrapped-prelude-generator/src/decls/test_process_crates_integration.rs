macro_rules! deps {
    () => {
        FileProcessingStatus!();
        Args!();
        FileProcessingResult!();
    };
}

macro_rules! test_process_crates_integration {
    () => {
        deps!();
        # [test] fn test_process_crates_integration () -> Result < () > { let temp_dir = tempdir () ? ; let project_root = temp_dir . path () . to_path_buf () ; let crate1_path = setup_test_crate (& project_root , "my-crate" , "use std::collections::HashMap;\nfn my_func() {}\n" ,) ; let args = Args { dry_run : false , path : project_root . clone () , exclude_crates : vec ! [] , report : false , results_file : Some (project_root . join ("results.json")) , cache_report : false , timeout : None , force : true , .. Default :: default () } ; process_crates (& args) ? ; let prelude_path = crate1_path . join ("src/prelude.rs") ; assert ! (prelude_path . exists ()) ; assert ! (fs :: read_to_string (& prelude_path) ?. contains ("// This is a generated prelude file")) ; let lib_rs_path = crate1_path . join ("src/lib.rs") ; let lib_rs_content = fs :: read_to_string (& lib_rs_path) ? ; assert ! (lib_rs_content . contains ("use crate::prelude::*")) ; assert ! (! lib_rs_content . contains ("use std::collections::HashMap;")) ; let results_file_content = fs :: read_to_string (& args . results_file . unwrap ()) ? ; let results : Vec < FileProcessingResult > = serde_json :: from_str (& results_file_content) ? ; assert_eq ! (results . len () , 2) ; assert ! (results . iter () . any (| r | r . path . ends_with ("src/lib.rs") && matches ! (r . status , FileProcessingStatus :: Success))) ; assert ! (results . iter () . any (| r | r . path . ends_with ("src/prelude.rs") && matches ! (r . status , FileProcessingStatus :: Success))) ; Ok (()) }
    };
}

test_process_crates_integration!()