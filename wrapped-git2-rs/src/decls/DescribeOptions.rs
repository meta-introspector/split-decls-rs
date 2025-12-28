macro_rules! deps {
    () => {
        Describe!();
    };
}

macro_rules! DescribeOptions {
    () => {
        deps!();
        # [doc = " Options which indicate how a `Describe` is created."] pub struct DescribeOptions { raw : raw :: git_describe_options , pattern : CString , }
    };
}

DescribeOptions!()