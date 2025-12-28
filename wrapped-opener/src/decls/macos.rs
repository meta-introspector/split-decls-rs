macro_rules! macos {
    () => {
        # [cfg (target_os = "macos")] mod macos ;
    };
}

macos!()