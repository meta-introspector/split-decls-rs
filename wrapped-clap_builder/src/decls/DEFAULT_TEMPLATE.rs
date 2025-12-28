macro_rules! DEFAULT_TEMPLATE {
    () => {
        const DEFAULT_TEMPLATE : & str = "\
{before-help}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}\
    " ;
    };
}

DEFAULT_TEMPLATE!()