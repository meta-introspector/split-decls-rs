macro_rules! symlink_supported {
    () => {
        # [cfg (not (windows))] pub fn symlink_supported () -> bool { true }
    };
}

symlink_supported!()