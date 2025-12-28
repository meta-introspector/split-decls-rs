macro_rules! deps {
    () => {
        IncrementCounter!();
    };
}

macro_rules! load_counters {
    () => {
        deps!();
        # [inline (always)] fn load_counters (counter : u64 , increment_counter : IncrementCounter) -> (v128 , v128) { let mask = if increment_counter . yes () { ! 0 } else { 0 } ; (set4 (counter_low (counter + (mask & 0)) , counter_low (counter + (mask & 1)) , counter_low (counter + (mask & 2)) , counter_low (counter + (mask & 3)) ,) , set4 (counter_high (counter + (mask & 0)) , counter_high (counter + (mask & 1)) , counter_high (counter + (mask & 2)) , counter_high (counter + (mask & 3)) ,) ,) }
    };
}

load_counters!()