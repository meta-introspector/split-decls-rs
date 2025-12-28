macro_rules! macro_1 {
    () => {
        include ! (concat ! (env ! ("OUT_DIR") , "/cargo_tree_data.rs")) ;
    };
}

macro_1!()