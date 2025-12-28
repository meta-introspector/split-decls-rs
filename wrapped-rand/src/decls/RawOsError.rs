macro_rules! RawOsError {
    () => {
        # [cfg (target_os = "uefi")] type RawOsError = usize ;
    };
}

RawOsError!();