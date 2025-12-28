macro_rules! deps {
    () => {
        Tag!();
        Object!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl From < Tag > for Object { fn from (v : Tag) -> Self { Object :: Tag (v) } }
    };
}

impl_59!()