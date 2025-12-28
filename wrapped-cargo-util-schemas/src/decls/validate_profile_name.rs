macro_rules! deps {
    () => {
        ErrorKind!();
        Result!();
    };
}

macro_rules! validate_profile_name {
    () => {
        deps!();
        # [doc = " Validate dir-names and profile names according to RFC 2678."] pub (crate) fn validate_profile_name (name : & str) -> Result < () > { if let Some (ch) = name . chars () . find (| ch | ! ch . is_alphanumeric () && * ch != '_' && * ch != '-') { return Err (ErrorKind :: InvalidCharacter { ch , what : "profile name" , name : name . into () , reason : "allowed characters are letters, numbers, underscore, and hyphen" , } . into ()) ; } let lower_name = name . to_lowercase () ; if lower_name == "debug" { return Err (ErrorKind :: ProfileNameReservedKeyword { name : name . into () , help : "To configure the default development profile, \
                use the name `dev` as in [profile.dev]" , } . into ()) ; } if lower_name == "build-override" { return Err (ErrorKind :: ProfileNameReservedKeyword { name : name . into () , help : "To configure build dependency settings, use [profile.dev.build-override] \
                 and [profile.release.build-override]" , } . into ()) ; } if matches ! (lower_name . as_str () , "build" | "check" | "clean" | "config" | "fetch" | "fix" | "install" | "metadata" | "package" | "publish" | "report" | "root" | "run" | "rust" | "rustc" | "rustdoc" | "target" | "tmp" | "uninstall") || lower_name . starts_with ("cargo") { return Err (ErrorKind :: ProfileNameReservedKeyword { name : name . into () , help : "Please choose a different name." , } . into ()) ; } Ok (()) }
    };
}

validate_profile_name!();