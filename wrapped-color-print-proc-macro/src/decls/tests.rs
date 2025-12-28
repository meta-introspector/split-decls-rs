macro_rules! tests {
    () => {
        # [cfg (test)] mod tests { use super :: * ; # [test] fn test_uppercase_word () { let input = "foo" ; assert ! (uppercase_word (input) . is_err ()) ; let input = "FOOfoo" ; assert ! (uppercase_word (input) . is_err ()) ; let input = "FOO" ; assert ! (uppercase_word (input) . is_ok ()) ; let input = "FOO;;" ; assert ! (uppercase_word (input) . is_ok ()) ; } }
    };
}

tests!()