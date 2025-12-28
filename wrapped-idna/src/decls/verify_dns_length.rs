macro_rules! verify_dns_length {
    () => {
        # [doc = " Performs the _VerifyDNSLength_ check on the output of the _ToASCII_ operation."] # [doc = ""] # [doc = " If the second argument is `false`, the trailing root label dot is allowed."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics in debug mode if the argument isn't ASCII."] pub fn verify_dns_length (domain_name : & str , allow_trailing_dot : bool) -> bool { let bytes = domain_name . as_bytes () ; debug_assert ! (bytes . is_ascii ()) ; let domain_name_without_trailing_dot = if let Some (without) = bytes . strip_suffix (b".") { if ! allow_trailing_dot { return false ; } without } else { bytes } ; if domain_name_without_trailing_dot . len () > 253 { return false ; } for label in domain_name_without_trailing_dot . split (| b | * b == b'.') { if label . is_empty () { return false ; } if label . len () > 63 { return false ; } } true }
    };
}

verify_dns_length!();