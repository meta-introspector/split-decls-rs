macro_rules! deps {
    () => {
        Object!();
        Tag!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl From < Tag > for Object { fn from (v : Tag) -> Self { Object :: Tag (v) } }
    };
}

impl_59!();