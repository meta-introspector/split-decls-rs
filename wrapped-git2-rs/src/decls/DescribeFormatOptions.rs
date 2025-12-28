macro_rules! DescribeFormatOptions {
    () => {
        # [doc = " Options which can be used to customize how a description is formatted."] pub struct DescribeFormatOptions { raw : raw :: git_describe_format_options , dirty_suffix : CString , }
    };
}

DescribeFormatOptions!();