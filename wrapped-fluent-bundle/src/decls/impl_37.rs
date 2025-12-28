macro_rules! deps {
    () => {
        FluentAttribute!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < 'm > From < & 'm ast :: Attribute < & 'm str > > for FluentAttribute < 'm > { fn from (attr : & 'm ast :: Attribute < & 'm str >) -> Self { FluentAttribute { node : attr } } }
    };
}

impl_37!();