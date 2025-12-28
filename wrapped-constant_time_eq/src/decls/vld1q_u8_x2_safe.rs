macro_rules! vld1q_u8_x2_safe {
    () => {
        # [doc = " Safe equivalent to `vld1q_u8_x2` for byte slices."] # [must_use] # [inline (always)] fn vld1q_u8_x2_safe (src : & [u8]) -> uint8x16x2_t { assert_eq ! (src . len () , size_of ::< uint8x16x2_t > ()) ; unsafe { vld1q_u8_x2 (src . as_ptr ()) } }
    };
}

vld1q_u8_x2_safe!()