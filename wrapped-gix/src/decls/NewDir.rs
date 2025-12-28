macro_rules! NewDir {
    () => {
        struct NewDir < 'a > (& 'a mut PathBuf) ;
    };
}

NewDir!()