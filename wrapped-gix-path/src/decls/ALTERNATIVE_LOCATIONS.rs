macro_rules! ALTERNATIVE_LOCATIONS {
    () => {
        # [cfg (not (windows))] pub (super) static ALTERNATIVE_LOCATIONS : LazyLock < Vec < PathBuf > > = LazyLock :: new (Vec :: new) ;
    };
}

ALTERNATIVE_LOCATIONS!();