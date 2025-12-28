macro_rules! PathCursor {
    () => {
        struct PathCursor < 'a > (& 'a mut PathBuf) ;
    };
}

PathCursor!()