macro_rules! deps {
    () => {
        EngineWrapper!();
    };
}

macro_rules! estimate_via_u128_inflation {
    () => {
        deps!();
        # [apply (all_engines)] fn estimate_via_u128_inflation < E : EngineWrapper > (engine_wrapper : E) { (0 .. 1000) . chain (usize :: MAX - 1000 ..= usize :: MAX) . for_each (| encoded_len | { let len_128 = encoded_len as u128 ; let estimate = E :: standard () . internal_decoded_len_estimate (encoded_len) . decoded_len_estimate () ; assert_eq ! (((len_128 + 3) / 4 * 3) as usize , estimate , "enc len {}" , encoded_len) ; }) }
    };
}

estimate_via_u128_inflation!()