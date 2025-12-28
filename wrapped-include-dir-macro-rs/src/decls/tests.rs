macro_rules! tests {
    () => {
        # [cfg (test)] mod tests { # [test] fn it_works () { assert_eq ! (2 + 2 , 4) ; } }
    };
}

tests!()