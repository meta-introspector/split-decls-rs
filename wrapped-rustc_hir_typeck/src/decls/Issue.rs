macro_rules! Issue {
    () => {
        # [derive (Debug)] enum Issue { # [doc = " The given argument is the invalid type for the input"] Invalid (usize) , # [doc = " There is a missing input"] Missing (usize) , # [doc = " There's a superfluous argument"] Extra (usize) , # [doc = " Two arguments should be swapped"] Swap (usize , usize) , # [doc = " Several arguments should be reordered"] Permutation (Vec < Option < usize > >) , }
    };
}

Issue!();