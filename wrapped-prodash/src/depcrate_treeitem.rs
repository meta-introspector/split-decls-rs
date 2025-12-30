// Generated macro for Item (struct)
macro_rules! Depcrate_treeItem {
() => {
// Module: crate::tree
// Provides: {"Item"}
// Dependencies: {}
# [doc = " A `Tree` represents an element of the progress tree."] # [doc = ""] # [doc = " It can be used to set progress and send messages."] # [doc = " ```rust"] # [doc = " let tree = prodash::tree::Root::new();"] # [doc = " let mut progress = tree.add_child(\"task 1\");"] # [doc = ""] # [doc = " progress.init(Some(10), Some(\"elements\".into()));"] # [doc = " for p in 0..10 {"] # [doc = "     progress.set(p);"] # [doc = " }"] # [doc = " progress.done(\"great success\");"] # [doc = " let mut  sub_progress = progress.add_child_with_id(\"sub-task 1\", *b\"TSK2\");"] # [doc = " sub_progress.init(None, None);"] # [doc = " sub_progress.set(5);"] # [doc = " sub_progress.fail(\"couldn't finish\");"] # [doc = " ```"] pub struct Item { pub (crate) key : crate :: progress :: Key , pub (crate) value : crate :: progress :: StepShared , pub (crate) highest_child_id : crate :: progress :: key :: Id , pub (crate) tree : std :: sync :: Arc < HashMap < crate :: progress :: Key , crate :: progress :: Task > > , pub (crate) messages : std :: sync :: Arc < parking_lot :: Mutex < MessageRingBuffer > > , }
};
}
