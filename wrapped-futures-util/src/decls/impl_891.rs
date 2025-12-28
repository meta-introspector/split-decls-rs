macro_rules! deps {
    () => {
        SelectAll!();
    };
}

macro_rules! impl_891 {
    () => {
        deps!();
        impl < St : Stream + Unpin > Default for SelectAll < St > { fn default () -> Self { Self :: new () } }
    };
}

impl_891!()