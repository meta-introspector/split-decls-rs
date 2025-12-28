macro_rules! deps {
    () => {
        Object!();
        Tree!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl From < Tree > for Object { fn from (v : Tree) -> Self { Object :: Tree (v) } }
    };
}

impl_61!()