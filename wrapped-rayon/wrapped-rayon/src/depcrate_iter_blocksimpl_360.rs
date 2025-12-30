// Generated macro for impl_360 (impl)
macro_rules! Depcrate_iter_blocksimpl_360 {
() => {
// Module: crate::iter::blocks
// Provides: {"impl_360"}
// Dependencies: {}
impl < T , S , C > ProducerCallback < T > for BlocksCallback < S , C > where C : UnindexedConsumer < T > , S : Iterator < Item = usize > , { type Output = C :: Result ; fn callback < P : Producer < Item = T > > (mut self , mut producer : P) -> Self :: Output { let mut remaining_len = self . len ; let mut consumer = self . consumer ; let (left_consumer , right_consumer , _) = consumer . split_at (0) ; let mut leftmost_res = left_consumer . into_folder () . complete () ; consumer = right_consumer ; while remaining_len > 0 && ! consumer . full () { let size = self . sizes . next () . unwrap_or (usize :: MAX) ; let capped_size = remaining_len . min (size) ; remaining_len -= capped_size ; let (left_producer , right_producer) = producer . split_at (capped_size) ; producer = right_producer ; let (left_consumer , right_consumer , _) = consumer . split_at (capped_size) ; consumer = right_consumer ; leftmost_res = consumer . to_reducer () . reduce (leftmost_res , bridge_producer_consumer (capped_size , left_producer , left_consumer) ,) ; } leftmost_res } }
};
}
