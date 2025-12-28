macro_rules! deps {
    () => {
        Project!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        # [doc = " This `Drop` implementation will clean up the temporary crates when expansion is finished."] # [doc = " This is to prevent pollution of the filesystem with dormant files."] impl Drop for Project { fn drop (& mut self) { if let Err (e) = fs :: remove_dir_all (& self . dir) { eprintln ! ("Failed to cleanup the directory `{}`: {}" , self . dir . to_string_lossy () , e) ; } } }
    };
}

impl_45!()