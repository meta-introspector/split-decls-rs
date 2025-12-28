macro_rules! Source {
    () => {
        # [doc = " The source of an additional entry"] pub enum Source { # [doc = " There is no content, typically the case with directories which are always considered empty."] Null , # [doc = " Read from the file at the given path."] Path (PathBuf) , # [doc = " Read from memory."] Memory (Vec < u8 >) , }
    };
}

Source!();