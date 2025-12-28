macro_rules! tests {
    () => {
        # [cfg (test)] mod tests { use super :: { escape_help , escape_value } ; # [test] fn test_escape_value () { let raw_string = "\\ [foo]() `bar https://$PATH" ; assert_eq ! (escape_value (raw_string) , "\\\\\\ \\[foo\\]\\(\\)\\ \\`bar\\ https\\://\\$PATH") ; } # [test] fn test_escape_help () { let raw_string = "\\ [foo]() `bar https://$PATH" ; assert_eq ! (escape_help (raw_string) , "\\\\ \\[foo\\]() \\`bar https\\://\\$PATH") ; } }
    };
}

tests!();