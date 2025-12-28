macro_rules! test_checked_binomial {
    () => {
        # [test] fn test_checked_binomial () { const LIMIT : usize = 500 ; let mut row = vec ! [Some (0) ; LIMIT + 1] ; row [0] = Some (1) ; for n in 0 ..= LIMIT { for k in 0 ..= LIMIT { assert_eq ! (row [k] , checked_binomial (n , k)) ; } row = std :: iter :: once (Some (1)) . chain ((1 ..= LIMIT) . map (| k | row [k - 1] ? . checked_add (row [k] ?))) . collect () ; } }
    };
}

test_checked_binomial!()