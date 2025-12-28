macro_rules! tests {
    () => {
        # [cfg (test)] mod tests { use super :: * ; # [test] fn add_lock_suffix_to_file_with_extension () { assert_eq ! (add_lock_suffix (Path :: new ("hello.ext")) , Path :: new ("hello.ext.lock")) ; } # [test] fn add_lock_suffix_to_file_without_extension () { assert_eq ! (add_lock_suffix (Path :: new ("hello")) , Path :: new ("hello.lock")) ; } }
    };
}

tests!()