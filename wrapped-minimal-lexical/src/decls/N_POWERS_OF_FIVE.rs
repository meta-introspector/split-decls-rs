macro_rules! N_POWERS_OF_FIVE {
    () => {
        pub const N_POWERS_OF_FIVE : usize = (LARGEST_POWER_OF_FIVE - SMALLEST_POWER_OF_FIVE + 1) as usize ;
    };
}

N_POWERS_OF_FIVE!()