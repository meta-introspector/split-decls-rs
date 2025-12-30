// Generated macro for macro_29 (macro)
macro_rules! Depcrate_fseventmacro_29 {
() => {
// Module: crate::fsevent
// Provides: {"macro_29"}
// Dependencies: {}
bitflags :: bitflags ! { # [repr (C)] # [derive (Debug)] struct StreamFlags : u32 { const NONE = fs :: kFSEventStreamEventFlagNone ; const MUST_SCAN_SUBDIRS = fs :: kFSEventStreamEventFlagMustScanSubDirs ; const USER_DROPPED = fs :: kFSEventStreamEventFlagUserDropped ; const KERNEL_DROPPED = fs :: kFSEventStreamEventFlagKernelDropped ; const IDS_WRAPPED = fs :: kFSEventStreamEventFlagEventIdsWrapped ; const HISTORY_DONE = fs :: kFSEventStreamEventFlagHistoryDone ; const ROOT_CHANGED = fs :: kFSEventStreamEventFlagRootChanged ; const MOUNT = fs :: kFSEventStreamEventFlagMount ; const UNMOUNT = fs :: kFSEventStreamEventFlagUnmount ; const ITEM_CREATED = fs :: kFSEventStreamEventFlagItemCreated ; const ITEM_REMOVED = fs :: kFSEventStreamEventFlagItemRemoved ; const INODE_META_MOD = fs :: kFSEventStreamEventFlagItemInodeMetaMod ; const ITEM_RENAMED = fs :: kFSEventStreamEventFlagItemRenamed ; const ITEM_MODIFIED = fs :: kFSEventStreamEventFlagItemModified ; const FINDER_INFO_MOD = fs :: kFSEventStreamEventFlagItemFinderInfoMod ; const ITEM_CHANGE_OWNER = fs :: kFSEventStreamEventFlagItemChangeOwner ; const ITEM_XATTR_MOD = fs :: kFSEventStreamEventFlagItemXattrMod ; const IS_FILE = fs :: kFSEventStreamEventFlagItemIsFile ; const IS_DIR = fs :: kFSEventStreamEventFlagItemIsDir ; const IS_SYMLINK = fs :: kFSEventStreamEventFlagItemIsSymlink ; const OWN_EVENT = fs :: kFSEventStreamEventFlagOwnEvent ; const IS_HARDLINK = fs :: kFSEventStreamEventFlagItemIsHardlink ; const IS_LAST_HARDLINK = fs :: kFSEventStreamEventFlagItemIsLastHardlink ; const ITEM_CLONED = fs :: kFSEventStreamEventFlagItemCloned ; } }
};
}
