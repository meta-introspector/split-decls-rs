macro_rules! dir_does_not_contain_target {
    () => {
        # [test] fn dir_does_not_contain_target () { assert ! (! super :: dir_contains_target (& Some ("x86_64-unknown-linux-gnu" . into ()) , Path :: new ("/project/target/debug/build/project-ea75983148559682/out") , None ,)) ; }
    };
}

dir_does_not_contain_target!()