macro_rules! deps {
    () => {
        DescribeOptions!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        impl Default for DescribeOptions { fn default () -> Self { Self :: new () } }
    };
}

impl_295!();