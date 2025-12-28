macro_rules! ThreadData {
    () => {
        struct ThreadData { parker : ThreadParker , queue_tail : Cell < * const ThreadData > , prev : Cell < * const ThreadData > , next : Cell < * const ThreadData > , }
    };
}

ThreadData!()