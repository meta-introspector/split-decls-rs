// Generated macro for counter_increase_times (function)
macro_rules! Depcrate_test_framework_streamcipher_interfacecounter_increase_times {
() => {
// Module: crate::test_framework::streamcipher_interface
// Provides: {"counter_increase_times"}
// Dependencies: {}
# [cfg (feature = "safe_api")] # [doc = " Given an input length `a` find out how many times"] # [doc = " the initial counter on encrypt()/decrypt() would"] # [doc = " increase."] fn counter_increase_times (a : f32) -> u32 { if a <= 64f32 { return 0 ; } let check_with_floor = (a / 64f32) . floor () ; let actual = a / 64f32 ; assert ! (actual >= check_with_floor) ; if actual > check_with_floor { (actual . ceil () as u32) - 1 } else { (actual as u32) - 1 } }
};
}
