macro_rules! deps {
    () => {
        BlocksCallback!();
        ProducerCallback!();
        UnindexedConsumer!();
        Producer!();
    };
}

macro_rules! impl_293 {
    () => {
        deps!();
        impl < T , S , C > ProducerCallback < T > for BlocksCallback < S , C > where C : UnindexedConsumer < T > , S : Iterator < Item = usize > , { type Output = C :: Result ; fn callback < P : Producer < Item = T > > (mut self , mut producer : P) -> Self :: Output { let mut remaining_len = self . len ; let mut consumer = self . consumer ; let (left_consumer , right_consumer , _) = consumer . split_at (0) ; let mut leftmost_res = left_consumer . into_folder () . complete () ; consumer = right_consumer ; while remaining_len > 0 && ! consumer . full () { let size = self . sizes . next () . unwrap_or (usize :: MAX) ; let capped_size = remaining_len . min (size) ; remaining_len -= capped_size ; let (left_producer , right_producer) = producer . split_at (capped_size) ; producer = right_producer ; let (left_consumer , right_consumer , _) = consumer . split_at (capped_size) ; consumer = right_consumer ; leftmost_res = consumer . to_reducer () . reduce (leftmost_res , bridge_producer_consumer (capped_size , left_producer , left_consumer) ,) ; } leftmost_res } }
    };
}

impl_293!();