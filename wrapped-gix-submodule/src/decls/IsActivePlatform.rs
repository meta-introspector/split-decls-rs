macro_rules! IsActivePlatform {
    () => {
        # [doc = " A platform to keep the state necessary to perform repeated active checks, created by [File::is_active_platform()]."] pub struct IsActivePlatform { pub (crate) search : Option < gix_pathspec :: Search > , }
    };
}

IsActivePlatform!();