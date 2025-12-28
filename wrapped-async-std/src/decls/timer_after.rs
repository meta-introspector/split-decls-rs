macro_rules! timer_after {
    () => {
        # [cfg (any (feature = "unstable" , feature = "default"))] pub (crate) fn timer_after (dur : std :: time :: Duration) -> timer :: Timer { Timer :: after (dur) }
    };
}

timer_after!()