macro_rules! deps {
    () => {
        NaiveEstimate!();
        Naive!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl NaiveEstimate { fn new (input_len : usize) -> Self { let rem = input_len % Naive :: DECODE_INPUT_CHUNK_SIZE ; let complete_chunk_len = input_len - rem ; Self { rem , complete_chunk_len , } } }
    };
}

impl_130!()