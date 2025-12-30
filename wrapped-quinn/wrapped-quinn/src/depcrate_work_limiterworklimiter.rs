// Generated macro for WorkLimiter (struct)
macro_rules! Depcrate_work_limiterWorkLimiter {
() => {
// Module: crate::work_limiter
// Provides: {"WorkLimiter"}
// Dependencies: {}
# [doc = " Limits the amount of time spent on a certain type of work in a cycle"] # [doc = ""] # [doc = " The limiter works dynamically: For a sampled subset of cycles it measures"] # [doc = " the time that is approximately required for fulfilling 1 work item, and"] # [doc = " calculates the amount of allowed work items per cycle."] # [doc = " The estimates are smoothed over all cycles where the exact duration is measured."] # [doc = ""] # [doc = " In cycles where no measurement is performed the previously determined work limit"] # [doc = " is used."] # [doc = ""] # [doc = " For the limiter the exact definition of a work item does not matter."] # [doc = " It could for example track the amount of transmitted bytes per cycle,"] # [doc = " or the amount of transmitted datagrams per cycle."] # [doc = " It will however work best if the required time to complete a work item is"] # [doc = " constant."] # [derive (Debug)] pub (crate) struct WorkLimiter { # [doc = " Whether to measure the required work time, or to use the previous estimates"] mode : Mode , # [doc = " The current cycle number"] cycle : u16 , # [doc = " The time the cycle started - only used in measurement mode"] start_time : Option < Instant > , # [doc = " How many work items have been completed in the cycle"] completed : usize , # [doc = " The amount of work items which are allowed for a cycle"] allowed : usize , # [doc = " The desired cycle time"] desired_cycle_time : Duration , # [doc = " The estimated and smoothed time per work item in nanoseconds"] smoothed_time_per_work_item_nanos : f64 , }
};
}
