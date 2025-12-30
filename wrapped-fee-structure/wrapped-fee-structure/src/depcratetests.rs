// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_calculate_memory_usage_cost () { let heap_cost = 99 ; const K : u32 = 1024 ; assert_eq ! (heap_cost , FeeStructure :: calculate_memory_usage_cost (31 * K , heap_cost)) ; assert_eq ! (heap_cost , FeeStructure :: calculate_memory_usage_cost (32 * K , heap_cost)) ; assert_eq ! (heap_cost * 2 , FeeStructure :: calculate_memory_usage_cost (33 * K , heap_cost)) ; assert_eq ! (heap_cost * 2 , FeeStructure :: calculate_memory_usage_cost (64 * K , heap_cost)) ; } }
};
}
