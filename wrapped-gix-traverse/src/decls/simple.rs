macro_rules! deps {
    () => {
        Simple!();
    };
}

macro_rules! simple {
    () => {
        deps!();
        # [doc = " Simple ancestors traversal, without the need to keep track of graph-state."] pub mod simple ;
    };
}

simple!();