// Generated macro for test (module)
macro_rules! Depcrate_arbitrary__std_synctest {
() => {
// Module: crate::arbitrary::_std::sync
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { no_panic_test ! (mutex => Mutex < u8 >, rw_lock => RwLock < u8 >, barrier => Barrier , barrier_wait_result => BarrierWaitResult , condvar => Condvar , once => Once , wait_timeout_result => WaitTimeoutResult , recv_error => RecvError , send_error => SendError < u8 >, recv_timeout_error => RecvTimeoutError , try_recv_error => TryRecvError , try_send_error => TrySendError < u8 >, rx_tx => (Sender < u8 >, Receiver < u8 >) , rx_txiter => (Sender < u8 >, IntoIter < u8 >) , syncrx_tx => (SyncSender < u8 >, Receiver < u8 >) , syncrx_txiter => (SyncSender < u8 >, IntoIter < u8 >)) ; }
};
}
