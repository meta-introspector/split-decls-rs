// Generated macro for Drops (struct)
macro_rules! Depcrate_drop_listDrops {
() => {
// Module: crate::drop_list
// Provides: {"Drops"}
// Dependencies: {}
# [doc = " Single drop item."] # [doc = " Drops associated value when invoked."] struct Drops { # [doc = " Number of values pointed by `ptr`."] count : usize , # [doc = " Drop function."] drop : unsafe fn (NonNull < Drops > , usize) , # [doc = " Next item in the list."] next : Option < NonNull < Self > > , }
};
}
