macro_rules! TempfileOrTemppath {
    () => {
        enum TempfileOrTemppath { Tempfile (NamedTempFile) , Temppath (TempPath) , }
    };
}

TempfileOrTemppath!();