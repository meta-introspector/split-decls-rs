macro_rules! macro_0 {
    () => {
        include ! (concat ! (env ! ("OUT_DIR") , "/generated.rs")) ;
    };
}

macro_0!()