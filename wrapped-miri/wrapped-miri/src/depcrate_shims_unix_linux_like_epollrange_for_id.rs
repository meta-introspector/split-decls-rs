// Generated macro for range_for_id (function)
macro_rules! Depcrate_shims_unix_linux_like_epollrange_for_id {
() => {
// Module: crate::shims::unix::linux_like::epoll
// Provides: {"range_for_id"}
// Dependencies: {}
# [doc = " Returns the range of all EpollEventKey for the given FD ID."] fn range_for_id (id : FdId) -> std :: ops :: RangeInclusive < EpollEventKey > { (id , 0) ..= (id , i32 :: MAX) }
};
}
