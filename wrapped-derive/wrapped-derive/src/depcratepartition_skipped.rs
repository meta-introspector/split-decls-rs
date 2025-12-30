// Generated macro for partition_skipped (function)
macro_rules! Depcratepartition_skipped {
() => {
// Module: crate
// Provides: {"partition_skipped"}
// Dependencies: {}
# [allow (clippy :: type_complexity)] fn partition_skipped (members : Vec < Member > , member_prefixes : Vec < Prefix > , member_skips : Vec < bool > ,) -> ((Vec < Member > , Vec < Prefix >) , (Vec < Member > , Vec < Prefix >)) { let mut members_not_skip : Vec < Member > = Vec :: new () ; let mut member_prefixes_not_skip : Vec < Prefix > = Vec :: new () ; let mut members_skip : Vec < Member > = Vec :: new () ; let mut member_prefixes_skip : Vec < Prefix > = Vec :: new () ; for ((member , prefix) , skip) in members . into_iter () . zip (member_prefixes) . zip (member_skips) { if ! skip { members_not_skip . push (member) ; member_prefixes_not_skip . push (prefix) ; } else { members_skip . push (member) ; member_prefixes_skip . push (prefix) ; } } ((members_not_skip , member_prefixes_not_skip) , (members_skip , member_prefixes_skip) ,) }
};
}
