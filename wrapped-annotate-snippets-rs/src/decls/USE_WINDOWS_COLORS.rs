macro_rules! USE_WINDOWS_COLORS {
    () => {
        const USE_WINDOWS_COLORS : bool = cfg ! (windows) && ! cfg ! (feature = "testing-colors") ;
    };
}

USE_WINDOWS_COLORS!();