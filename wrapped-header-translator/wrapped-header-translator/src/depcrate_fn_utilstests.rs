// Generated macro for tests (module)
macro_rules! Depcrate_fn_utilstests {
() => {
// Module: crate::fn_utils
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_follows_create_rule () { assert ! (follows_create_rule ("ThingCreate")) ; assert ! (follows_create_rule ("CreateThing")) ; assert ! (follows_create_rule ("CopyCreateThing")) ; assert ! (follows_create_rule ("create_thing")) ; assert ! (! follows_create_rule ("Created")) ; assert ! (! follows_create_rule ("created")) ; assert ! (! follows_create_rule ("GetAbc")) ; assert ! (! follows_create_rule ("recreate")) ; assert ! (follows_create_rule ("CreatedCopy")) ; assert ! (follows_create_rule ("dispatch_data_create")) ; assert ! (follows_create_rule ("dispatch_data_create_map")) ; assert ! (! follows_create_rule ("dispatch_data_get_size")) ; assert ! (follows_create_rule ("MTLCreateSystemDefaultDevice")) ; assert ! (follows_create_rule ("MTLCopyAllDevices")) ; assert ! (! follows_create_rule ("MTLRemoveDeviceObserver")) ; assert ! (follows_create_rule ("CFArrayCreate")) ; assert ! (follows_create_rule ("CFArrayCreateCopy")) ; assert ! (! follows_create_rule ("CFArrayGetCount")) ; assert ! (! follows_create_rule ("CFArrayGetValues")) ; } }
};
}
