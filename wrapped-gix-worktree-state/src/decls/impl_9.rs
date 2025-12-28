macro_rules! deps {
    () => {
        Options!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl From < & checkout :: Options > for Options { fn from (opts : & checkout :: Options) -> Self { Options { fs : opts . fs , destination_is_initially_empty : opts . destination_is_initially_empty , overwrite_existing : opts . overwrite_existing , keep_going : opts . keep_going , filter_process_delay : opts . filter_process_delay , } } }
    };
}

impl_9!()