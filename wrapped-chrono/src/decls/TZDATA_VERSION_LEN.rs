macro_rules! TZDATA_VERSION_LEN {
    () => {
        # [doc = " Size of the version string in the header of `tzdata` file."] # [doc = " e.g. `tzdata2024b\\0`"] const TZDATA_VERSION_LEN : usize = 12 ;
    };
}

TZDATA_VERSION_LEN!()