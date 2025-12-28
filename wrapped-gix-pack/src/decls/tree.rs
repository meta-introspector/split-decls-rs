macro_rules! deps {
    () => {
        Tree!();
    };
}

macro_rules! tree {
    () => {
        deps!();
        # [doc = " Tree datastructure"] mod tree ;
    };
}

tree!();