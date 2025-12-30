// Generated macro for impl_1705 (impl)
macro_rules! Depcrate_vecimpl_1705 {
() => {
// Module: crate::vec
// Provides: {"impl_1705"}
// Dependencies: {}
impl < 'data , T : Send > IndexedParallelIterator for Drain < 'data , T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { bridge (self , consumer) } fn len (& self) -> usize { self . range . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { unsafe { self . vec . set_len (self . range . start) ; let producer = DrainProducer :: from_vec (self . vec , self . range . len ()) ; callback . callback (producer) } } }
};
}
