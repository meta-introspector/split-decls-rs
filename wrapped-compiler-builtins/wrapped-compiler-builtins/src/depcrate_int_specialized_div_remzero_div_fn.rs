// Generated macro for zero_div_fn (function)
macro_rules! Depcrate_int_specialized_div_remzero_div_fn {
() => {
// Module: crate::int::specialized_div_rem
// Provides: {"zero_div_fn"}
// Dependencies: {}
# [doc = " The behavior of all divisions by zero is controlled by this function. This function should be"] # [doc = " impossible to reach by Rust users, unless `compiler-builtins` public division functions or"] # [doc = " `core/std::unchecked_div/rem` are directly used without a zero check in front."] fn zero_div_fn () -> ! { unsafe { core :: intrinsics :: unreachable () } }
};
}
