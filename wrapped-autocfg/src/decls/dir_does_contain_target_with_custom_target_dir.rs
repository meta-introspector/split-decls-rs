macro_rules! dir_does_contain_target_with_custom_target_dir {
    () => {
        # [test] fn dir_does_contain_target_with_custom_target_dir () { assert ! (super :: dir_contains_target (& Some ("x86_64-unknown-linux-gnu" . into ()) , Path :: new ("/project/custom/x86_64-unknown-linux-gnu/debug/build/project-0147aca016480b9d/out") , Some ("custom" . into ()) ,)) ; }
    };
}

dir_does_contain_target_with_custom_target_dir!();