macro_rules! EdgeType {
    () => {
        # [doc = " A graph's edge type determines whether it has directed edges or not."] pub trait EdgeType { fn is_directed () -> bool ; }
    };
}

EdgeType!();