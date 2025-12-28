macro_rules! deps {
    () => {
        Name!();
        String!();
    };
}

macro_rules! impl_914 {
    () => {
        deps!();
        impl From < BString > for Name < 'static > { fn from (name : BString) -> Self { Self :: try_from (Cow :: Owned (name)) . expect ("String is never illformed") } }
    };
}

impl_914!()