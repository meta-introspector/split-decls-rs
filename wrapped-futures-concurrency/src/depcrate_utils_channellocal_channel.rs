// Generated macro for local_channel (function)
macro_rules! Depcrate_utils_channellocal_channel {
() => {
// Module: crate::utils::channel
// Provides: {"local_channel"}
// Dependencies: {}
pub (crate) fn local_channel < T > () -> (LocalSender < T > , LocalReceiver < T >) { let channel = Rc :: new (RefCell :: new (LocalChannel { queue : VecDeque :: new () , waker : None , closed : false , })) ; (LocalSender { channel : channel . clone () , } , LocalReceiver { channel } ,) }
};
}
