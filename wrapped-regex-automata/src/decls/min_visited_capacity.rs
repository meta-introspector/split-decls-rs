macro_rules! deps {
    () => {
        Input!();
        NFA!();
        BoundedBacktracker!();
    };
}

macro_rules! min_visited_capacity {
    () => {
        deps!();
        # [doc = " Returns the minimum visited capacity for the given haystack."] # [doc = ""] # [doc = " This function can be used as the argument to [`Config::visited_capacity`]"] # [doc = " in order to guarantee that a backtracking search for the given `input`"] # [doc = " won't return an error when using a [`BoundedBacktracker`] built from the"] # [doc = " given `NFA`."] # [doc = ""] # [doc = " This routine exists primarily as a way to test that the bounded backtracker"] # [doc = " works correctly when its capacity is set to the smallest possible amount."] # [doc = " Still, it may be useful in cases where you know you want to use the bounded"] # [doc = " backtracker for a specific input, and just need to know what visited"] # [doc = " capacity to provide to make it work."] # [doc = ""] # [doc = " Be warned that this number could be quite large as it is multiplicative in"] # [doc = " the size the given NFA and haystack."] pub fn min_visited_capacity (nfa : & NFA , input : & Input < '_ >) -> usize { div_ceil (nfa . states () . len () * (input . get_span () . len () + 1) , 8) }
    };
}

min_visited_capacity!();