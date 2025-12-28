macro_rules! DEFAULT_NO_ARGS_TEMPLATE {
    () => {
        const DEFAULT_NO_ARGS_TEMPLATE : & str = "\
{before-help}{about-with-newline}
{usage-heading} {usage}{after-help}\
    " ;
    };
}

DEFAULT_NO_ARGS_TEMPLATE!()