macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! Optimum {
    () => {
        deps!();
        # [derive (Debug , Default , Clone)] struct Optimum { state : State , reps : [i32 ; REPS] , price : u32 , opt_prev : usize , back_prev : i32 , prev1_is_literal : bool , has_prev2 : bool , opt_prev2 : usize , back_prev2 : i32 , }
    };
}

Optimum!();