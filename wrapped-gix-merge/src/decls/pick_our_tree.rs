macro_rules! deps {
    () => {
        ConflictMapping!();
        TreeNodes!();
    };
}

macro_rules! pick_our_tree {
    () => {
        deps!();
        fn pick_our_tree < 'a > (side : ConflictMapping , ours : & 'a mut TreeNodes , theirs : & 'a mut TreeNodes) -> & 'a mut TreeNodes { match side { Original => ours , Swapped => theirs , } }
    };
}

pick_our_tree!();