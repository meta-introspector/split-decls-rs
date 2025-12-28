macro_rules! deps {
    () => {
        DescribeFormatOptions!();
    };
}

macro_rules! impl_293 {
    () => {
        deps!();
        impl Default for DescribeFormatOptions { fn default () -> Self { Self :: new () } }
    };
}

impl_293!();