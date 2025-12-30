// Generated macro for round_up_to_alignment (function)
macro_rules! Depcrate_transforms_multi_valueround_up_to_alignment {
() => {
// Module: crate::transforms::multi_value
// Provides: {"round_up_to_alignment"}
// Dependencies: {}
fn round_up_to_alignment (n : u32 , align : u32) -> u32 { debug_assert ! (align . is_power_of_two ()) ; (n + align - 1) & ! (align - 1) }
};
}
