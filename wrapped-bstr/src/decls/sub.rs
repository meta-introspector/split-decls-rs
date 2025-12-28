macro_rules! sub {
    () => {
        # [doc = " Subtract `b` from `a` and return the difference. `a` should be greater than"] # [doc = " or equal to `b`."] fn sub (a : * const u8 , b : * const u8) -> usize { debug_assert ! (a >= b) ; (a as usize) - (b as usize) }
    };
}

sub!();