macro_rules! GraphRef {
    () => {
        # [doc = " A copyable reference to a graph."] pub trait GraphRef : Copy + GraphBase { }
    };
}

GraphRef!();