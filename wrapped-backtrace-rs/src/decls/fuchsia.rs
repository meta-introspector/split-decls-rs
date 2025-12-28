macro_rules! fuchsia {
    () => {
        # [cfg (target_os = "fuchsia")] mod fuchsia ;
    };
}

fuchsia!();