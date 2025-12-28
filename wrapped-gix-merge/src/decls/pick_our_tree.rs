macro_rules! deps {
    () => {
        TreeNodes!();
        ConflictMapping!();
    };
}

macro_rules! pick_our_tree {
    () => {
        deps!();
        fn pick_our_tree < 'a > (side : ConflictMapping , ours : & 'a mut TreeNodes , theirs : & 'a mut TreeNodes) -> & 'a mut TreeNodes { match side { Original => ours , Swapped => theirs , } }
    };
}

pick_our_tree!()