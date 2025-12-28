macro_rules! Retries {
    () => {
        # [doc = " The amount of retries to do during various aspects of the directory creation."] # [derive (Debug , Clone , Copy , Ord , PartialOrd , Eq , PartialEq)] pub struct Retries { # [doc = " How many times the whole directory can be created in the light of racy interference."] # [doc = " This count combats racy situations where another process is trying to remove a directory that we want to create,"] # [doc = " and is deliberately higher than those who do deletion. That way, creation usually wins."] pub to_create_entire_directory : usize , # [doc = " The amount of times we can try to create a directory because we couldn't as the parent didn't exist."] # [doc = " This amounts to the maximum subdirectory depth we allow to be created. Counts once per attempt to create the entire directory."] pub on_create_directory_failure : usize , # [doc = " How often to retry to create a single directory if an interrupt happens, as caused by signals."] pub on_interrupt : usize , }
    };
}

Retries!()