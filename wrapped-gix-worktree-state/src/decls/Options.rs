macro_rules! Options {
    () => {
        # [derive (Clone , Copy)] pub struct Options { pub fs : gix_fs :: Capabilities , pub destination_is_initially_empty : bool , pub overwrite_existing : bool , pub keep_going : bool , pub filter_process_delay : gix_filter :: driver :: apply :: Delay , }
    };
}

Options!();