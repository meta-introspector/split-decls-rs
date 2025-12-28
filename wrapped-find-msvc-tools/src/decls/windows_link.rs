macro_rules! windows_link {
    () => {
        # [cfg (windows)] # [doc (hidden)] pub mod windows_link ;
    };
}

windows_link!();