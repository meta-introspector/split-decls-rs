macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! deserialize_rust_version {
    () => {
        deps!();
        # [doc = " As per the Cargo Book the [`rust-version` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field) must:"] # [doc = ""] # [doc = " > be a bare version number with two or three components;"] # [doc = " > it cannot include semver operators or pre-release identifiers."] # [doc = ""] # [doc = " [`semver::Version`] however requires three components. This function takes"] # [doc = " care of appending `.0` if the provided version number only has two components"] # [doc = " and ensuring that it does not contain a pre-release version or build metadata."] fn deserialize_rust_version < 'de , D > (deserializer : D ,) -> std :: result :: Result < Option < Version > , D :: Error > where D : Deserializer < 'de > , { let mut buf = match Option :: < String > :: deserialize (deserializer) ? { None => return Ok (None) , Some (buf) => buf , } ; for char in buf . chars () { if char == '-' { return Err (serde :: de :: Error :: custom ("pre-release identifiers are not supported in rust-version" ,)) ; } else if char == '+' { return Err (serde :: de :: Error :: custom ("build metadata is not supported in rust-version" ,)) ; } } if buf . matches ('.') . count () == 1 { buf . push_str (".0") ; } Ok (Some (Version :: parse (& buf) . map_err (serde :: de :: Error :: custom) ? ,)) }
    };
}

deserialize_rust_version!();