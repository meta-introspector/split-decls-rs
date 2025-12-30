// Generated macro for round (function)
macro_rules! Depcrate_portableround {
() => {
// Module: crate::portable
// Provides: {"round"}
// Dependencies: {}
# [inline (always)] fn round (state : & mut [u32 ; 16] , msg : & [u32 ; 16] , round : usize) { let schedule = MSG_SCHEDULE [round] ; g (state , 0 , 4 , 8 , 12 , msg [schedule [0]] , msg [schedule [1]]) ; g (state , 1 , 5 , 9 , 13 , msg [schedule [2]] , msg [schedule [3]]) ; g (state , 2 , 6 , 10 , 14 , msg [schedule [4]] , msg [schedule [5]]) ; g (state , 3 , 7 , 11 , 15 , msg [schedule [6]] , msg [schedule [7]]) ; g (state , 0 , 5 , 10 , 15 , msg [schedule [8]] , msg [schedule [9]]) ; g (state , 1 , 6 , 11 , 12 , msg [schedule [10]] , msg [schedule [11]]) ; g (state , 2 , 7 , 8 , 13 , msg [schedule [12]] , msg [schedule [13]]) ; g (state , 3 , 4 , 9 , 14 , msg [schedule [14]] , msg [schedule [15]]) ; }
};
}
