macro_rules! dir_does_not_contain_target_with_custom_target_dir {
    () => {
        # [test] fn dir_does_not_contain_target_with_custom_target_dir () { assert ! (! super :: dir_contains_target (& Some ("x86_64-unknown-linux-gnu" . into ()) , Path :: new ("/project/custom/debug/build/project-ea75983148559682/out") , Some ("custom" . into ()) ,)) ; }
    };
}

dir_does_not_contain_target_with_custom_target_dir!()