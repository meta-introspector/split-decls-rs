macro_rules! windows {
    () => {
        # [cfg (target_os = "windows")] mod windows ;
    };
}

windows!()