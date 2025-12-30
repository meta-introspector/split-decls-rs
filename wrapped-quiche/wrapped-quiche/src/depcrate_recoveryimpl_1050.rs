// Generated macro for impl_1050 (impl)
macro_rules! Depcrate_recoveryimpl_1050 {
() => {
// Module: crate::recovery
// Provides: {"impl_1050"}
// Dependencies: {}
impl std :: fmt :: Debug for Sent { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { write ! (f , "pkt_num={:?} " , self . pkt_num) ? ; write ! (f , "pkt_sent_time={:?} " , self . time_sent) ? ; write ! (f , "pkt_size={:?} " , self . size) ? ; write ! (f , "delivered={:?} " , self . delivered) ? ; write ! (f , "delivered_time={:?} " , self . delivered_time) ? ; write ! (f , "first_sent_time={:?} " , self . first_sent_time) ? ; write ! (f , "is_app_limited={} " , self . is_app_limited) ? ; write ! (f , "tx_in_flight={} " , self . tx_in_flight) ? ; write ! (f , "lost={} " , self . lost) ? ; write ! (f , "has_data={} " , self . has_data) ? ; write ! (f , "is_pmtud_probe={}" , self . is_pmtud_probe) ? ; Ok (()) } }
};
}
