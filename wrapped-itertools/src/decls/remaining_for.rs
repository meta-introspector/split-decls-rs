macro_rules! remaining_for {
    () => {
        fn remaining_for (n : usize , k : usize) -> Option < usize > { (k + 1 ..= n) . try_fold (0usize , | sum , i | sum . checked_add (checked_binomial (n , i) ?)) }
    };
}

remaining_for!()