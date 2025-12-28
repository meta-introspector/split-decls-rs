macro_rules! deps {
    () => {
        FluentMessage!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < 'm > From < & 'm ast :: Message < & 'm str > > for FluentMessage < 'm > { fn from (msg : & 'm ast :: Message < & 'm str >) -> Self { FluentMessage { node : msg } } }
    };
}

impl_40!();