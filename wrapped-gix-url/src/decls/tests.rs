macro_rules! deps {
    () => {
        ParsedUrl!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use super :: * ; # [test] fn test_simple_url () { let url = ParsedUrl :: parse ("http://example.com/path") . unwrap () ; assert_eq ! (url . scheme , "http") ; assert_eq ! (url . host . as_deref () , Some ("example.com")) ; assert_eq ! (url . path , "/path") ; assert_eq ! (url . username , "") ; assert_eq ! (url . password , None) ; assert_eq ! (url . port , None) ; } # [test] fn test_url_with_port () { let url = ParsedUrl :: parse ("http://example.com:8080/path") . unwrap () ; assert_eq ! (url . scheme , "http") ; assert_eq ! (url . host . as_deref () , Some ("example.com")) ; assert_eq ! (url . port , Some (8080)) ; assert_eq ! (url . path , "/path") ; } # [test] fn test_url_with_user () { let url = ParsedUrl :: parse ("http://user@example.com/path") . unwrap () ; assert_eq ! (url . scheme , "http") ; assert_eq ! (url . username , "user") ; assert_eq ! (url . host . as_deref () , Some ("example.com")) ; assert_eq ! (url . path , "/path") ; } # [test] fn test_url_with_user_and_password () { let url = ParsedUrl :: parse ("http://user:pass@example.com/path") . unwrap () ; assert_eq ! (url . scheme , "http") ; assert_eq ! (url . username , "user") ; assert_eq ! (url . password , Some ("pass")) ; assert_eq ! (url . host . as_deref () , Some ("example.com")) ; assert_eq ! (url . path , "/path") ; } # [test] fn test_url_with_ipv6 () { let url = ParsedUrl :: parse ("http://[::1]/path") . unwrap () ; assert_eq ! (url . scheme , "http") ; assert_eq ! (url . host . as_deref () , Some ("[::1]")) ; assert_eq ! (url . path , "/path") ; } # [test] fn test_url_with_ipv6_and_port () { let url = ParsedUrl :: parse ("http://[::1]:8080/path") . unwrap () ; assert_eq ! (url . scheme , "http") ; assert_eq ! (url . host . as_deref () , Some ("[::1]")) ; assert_eq ! (url . port , Some (8080)) ; assert_eq ! (url . path , "/path") ; } }
    };
}

tests!()