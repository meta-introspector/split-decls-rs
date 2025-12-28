macro_rules! short_describe_prefix {
    () => {
        fn short_describe_prefix (name : & BStr) -> Option < & BStr > { let mut iter = name . split (| b | * b == b'-') ; let candidate = iter . next () . and_then (| prefix | prefix . iter () . all (u8 :: is_ascii_hexdigit) . then (| | prefix . as_bstr ())) ; (iter . count () == 1) . then_some (candidate) . flatten () }
    };
}

short_describe_prefix!();