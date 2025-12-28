macro_rules! deps {
    () => {
        Score!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl Score { fn is_improvement_over (self , prev_score : Self) -> bool { let indent_score = match prev_score . indent . cmp (& self . indent) { Ordering :: Less => INDENT_WEIGHT , Ordering :: Greater => - INDENT_WEIGHT , Ordering :: Equal => 0 , } ; (indent_score + self . penalty - prev_score . penalty) <= 0 } }
    };
}

impl_60!();