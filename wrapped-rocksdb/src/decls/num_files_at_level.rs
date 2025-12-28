macro_rules! deps {
    () => {
        PropertyName!();
    };
}

macro_rules! num_files_at_level {
    () => {
        deps!();
        # [doc = " \"rocksdb.num-files-at-level<`N`>\" - returns string containing the number"] # [doc = " of files at level <`N`>, where <`N`> is an ASCII representation of a"] # [doc = " level number (e.g., \"0\")."] pub fn num_files_at_level (level : usize) -> PropertyName { unsafe { level_property ("num-files-at-level" , level) } }
    };
}

num_files_at_level!()