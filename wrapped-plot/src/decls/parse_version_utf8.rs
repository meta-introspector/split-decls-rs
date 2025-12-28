macro_rules! deps {
    () => {
        VersionError!();
        Version!();
    };
}

macro_rules! parse_version_utf8 {
    () => {
        deps!();
        fn parse_version_utf8 (output_bytes : & [u8]) -> Result < Version , VersionError > { let output = str :: from_utf8 (output_bytes) . map_err (| _ | VersionError :: OutputError) ? ; parse_version (output) . map_err (| _ | VersionError :: ParseError (output . to_owned ())) }
    };
}

parse_version_utf8!()