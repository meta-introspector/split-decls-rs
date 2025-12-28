macro_rules! deps {
    () => {
        Version!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl Version { # [doc = " Constructs `VersionType` from the given string."] # [doc = ""] # [doc = " Returns `VersionType::Unknown` if the string is empty. If it can be parsed as a semantic"] # [doc = " version, then `VersionType::Semantic`, otherwise `VersionType::Custom`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use os_info::Version;"] # [doc = ""] # [doc = " let v = Version::from_string(\"custom\");"] # [doc = " assert_eq!(Version::Custom(\"custom\".to_owned()), v);"] # [doc = ""] # [doc = " let v = Version::from_string(\"1.2.3\");"] # [doc = " assert_eq!(Version::Semantic(1, 2, 3), v);"] # [doc = " ```"] pub fn from_string < S : Into < String > + AsRef < str > > (s : S) -> Self { if s . as_ref () . is_empty () { Self :: Unknown } else if let Some ((major , minor , patch)) = parse_version (s . as_ref ()) { Self :: Semantic (major , minor , patch) } else { Self :: Custom (s . into ()) } } }
    };
}

impl_52!()