macro_rules! macro_5 {
    () => {
        include ! (concat ! (env ! ("OUT_DIR") , "/bindings.rs")) ;
    };
}

macro_5!();