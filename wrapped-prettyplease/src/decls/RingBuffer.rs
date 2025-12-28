macro_rules! RingBuffer {
    () => {
        pub struct RingBuffer < T > { data : VecDeque < T > , offset : usize , }
    };
}

RingBuffer!()