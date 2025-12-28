macro_rules! HardLinkFailed {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_hard_link_failed)] pub (crate) struct HardLinkFailed < 'a > { pub path : & 'a Path , }
    };
}

HardLinkFailed!();