macro_rules! deps {
    () => {
        InMemoryDir!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl From < () > for InMemoryDir { fn from (_files : ()) -> Self { let files = Vec :: new () ; Self { files } } }
    };
}

impl_26!();