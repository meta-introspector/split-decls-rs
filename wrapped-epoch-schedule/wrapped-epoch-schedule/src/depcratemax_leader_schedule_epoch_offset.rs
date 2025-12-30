// Generated macro for MAX_LEADER_SCHEDULE_EPOCH_OFFSET (const)
macro_rules! DepcrateMAX_LEADER_SCHEDULE_EPOCH_OFFSET {
() => {
// Module: crate
// Provides: {"MAX_LEADER_SCHEDULE_EPOCH_OFFSET"}
// Dependencies: {}
# [doc = " The maximum number of slots before an epoch starts to calculate the leader schedule."] # [doc = ""] # [doc = " Default is an entire epoch, i.e. leader schedule for epoch X is calculated at"] # [doc = " the beginning of epoch X - 1."] pub const MAX_LEADER_SCHEDULE_EPOCH_OFFSET : u64 = 3 ;
};
}
