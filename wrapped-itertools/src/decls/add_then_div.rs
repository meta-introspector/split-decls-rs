macro_rules! add_then_div {
    () => {
        # [doc = " `(n + a) / d` avoiding overflow when possible, returns `None` if it overflows."] fn add_then_div (n : usize , a : usize , d : usize) -> Option < usize > { debug_assert_ne ! (d , 0) ; (n / d) . checked_add (a / d) ? . checked_add ((n % d + a % d) / d) }
    };
}

add_then_div!();