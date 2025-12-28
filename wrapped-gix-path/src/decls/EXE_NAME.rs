macro_rules! EXE_NAME {
    () => {
        # [cfg (not (windows))] pub (super) const EXE_NAME : & str = "git" ;
    };
}

EXE_NAME!()