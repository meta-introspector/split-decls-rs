macro_rules! deps {
    () => {
        MaxRecursionReached!();
        Result!();
    };
}

macro_rules! try_recursion_guard {
    () => {
        deps!();
        # [doc = " Protects against potential infinite recursion when calculating size hints"] # [doc = " due to indirect type recursion."] # [doc = ""] # [doc = " When the depth is not too deep, calls `f` with `depth + 1` to calculate the"] # [doc = " size hint."] # [doc = ""] # [doc = " Otherwise, returns an error."] # [doc = ""] # [doc = " This should be used when implementing [`try_size_hint`](crate::Arbitrary::try_size_hint)"] # [inline] pub fn try_recursion_guard (depth : usize , f : impl FnOnce (usize) -> Result < (usize , Option < usize >) , crate :: MaxRecursionReached > ,) -> Result < (usize , Option < usize >) , crate :: MaxRecursionReached > { if depth > MAX_DEPTH { Err (crate :: MaxRecursionReached { }) } else { f (depth + 1) } }
    };
}

try_recursion_guard!();