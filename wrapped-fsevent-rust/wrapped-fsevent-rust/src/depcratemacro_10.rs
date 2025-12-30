// Generated macro for macro_10 (macro)
macro_rules! Depcratemacro_10 {
() => {
// Module: crate
// Provides: {"macro_10"}
// Dependencies: {}
bitflags ! { # [repr (C)] pub struct StreamFlags : u32 { const NONE = kFSEventStreamEventFlagNone ; const MUST_SCAN_SUBDIRS = kFSEventStreamEventFlagMustScanSubDirs ; const USER_DROPPED = kFSEventStreamEventFlagUserDropped ; const KERNEL_DROPPED = kFSEventStreamEventFlagKernelDropped ; const IDS_WRAPPED = kFSEventStreamEventFlagEventIdsWrapped ; const HISTORY_DONE = kFSEventStreamEventFlagHistoryDone ; const ROOT_CHANGED = kFSEventStreamEventFlagRootChanged ; const MOUNT = kFSEventStreamEventFlagMount ; const UNMOUNT = kFSEventStreamEventFlagUnmount ; const ITEM_CREATED = kFSEventStreamEventFlagItemCreated ; const ITEM_REMOVED = kFSEventStreamEventFlagItemRemoved ; const INODE_META_MOD = kFSEventStreamEventFlagItemInodeMetaMod ; const ITEM_RENAMED = kFSEventStreamEventFlagItemRenamed ; const ITEM_MODIFIED = kFSEventStreamEventFlagItemModified ; const FINDER_INFO_MOD = kFSEventStreamEventFlagItemFinderInfoMod ; const ITEM_CHANGE_OWNER = kFSEventStreamEventFlagItemChangeOwner ; const ITEM_XATTR_MOD = kFSEventStreamEventFlagItemXattrMod ; const IS_FILE = kFSEventStreamEventFlagItemIsFile ; const IS_DIR = kFSEventStreamEventFlagItemIsDir ; const IS_SYMLINK = kFSEventStreamEventFlagItemIsSymlink ; const OWN_EVENT = kFSEventStreamEventFlagOwnEvent ; const IS_HARDLINK = kFSEventStreamEventFlagItemIsHardlink ; const IS_LAST_HARDLINK = kFSEventStreamEventFlagItemIsLastHardlink ; const ITEM_CLONED = kFSEventStreamEventFlagItemCloned ; } }
};
}
