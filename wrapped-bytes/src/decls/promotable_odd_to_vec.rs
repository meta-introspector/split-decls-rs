macro_rules! promotable_odd_to_vec {
    () => {
        unsafe fn promotable_odd_to_vec (data : & AtomicPtr < () > , ptr : * const u8 , len : usize) -> Vec < u8 > { promotable_to_vec (data , ptr , len , | shared | shared . cast ()) }
    };
}

promotable_odd_to_vec!()