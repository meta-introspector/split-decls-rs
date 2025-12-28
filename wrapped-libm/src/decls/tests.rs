macro_rules! tests {
    () => {
        # [cfg (test)] mod tests { # [test] fn sanity_check () { assert_eq ! (super :: truncf (1.1) , 1.0) ; } }
    };
}

tests!();