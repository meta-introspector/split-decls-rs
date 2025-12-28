macro_rules! deps {
    () => {
        InMemoryDir!();
    };
}

macro_rules! validate_crate_contents_ {
    () => {
        deps!();
        # [track_caller] fn validate_crate_contents_ (reader : impl Read , expected_crate_name : & str , expected_files : & [& str] , expected_contents : InMemoryDir ,) { let mut rdr = GzDecoder :: new (reader) ; snapbox :: assert_data_eq ! (rdr . header () . unwrap () . filename () . unwrap () , { let expected : snapbox :: Data = expected_crate_name . into () ; expected . raw () }) ; let mut contents = Vec :: new () ; rdr . read_to_end (& mut contents) . unwrap () ; let mut ar = Archive :: new (& contents [..]) ; let base_crate_name = Path :: new (expected_crate_name . strip_suffix (".crate") . expect ("must end with .crate") ,) ; let actual_contents : InMemoryDir = ar . entries () . unwrap () . map (| entry | { let mut entry = entry . unwrap () ; let name = entry . path () . unwrap () . strip_prefix (base_crate_name) . unwrap () . to_owned () ; let mut contents = String :: new () ; entry . read_to_string (& mut contents) . unwrap () ; (name , contents) }) . collect () ; let actual_files : HashSet < & Path > = actual_contents . paths () . collect () ; let expected_files : HashSet < & Path > = expected_files . iter () . map (| name | Path :: new (name)) . collect () ; let missing : Vec < & & Path > = expected_files . difference (& actual_files) . collect () ; let extra : Vec < & & Path > = actual_files . difference (& expected_files) . collect () ; if ! missing . is_empty () || ! extra . is_empty () { panic ! ("uploaded archive does not match.\nMissing: {:?}\nExtra: {:?}\n" , missing , extra) ; } actual_contents . assert_contains (& expected_contents) ; }
    };
}

validate_crate_contents_!()