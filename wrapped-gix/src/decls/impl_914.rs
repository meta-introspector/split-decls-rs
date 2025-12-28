macro_rules! deps {
    () => {
        String!();
        Name!();
    };
}

macro_rules! impl_914 {
    () => {
        deps!();
        impl From < BString > for Name < 'static > { fn from (name : BString) -> Self { Self :: try_from (Cow :: Owned (name)) . expect ("String is never illformed") } }
    };
}

impl_914!();