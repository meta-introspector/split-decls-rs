macro_rules! BytesProducer {
    () => {
        struct BytesProducer < 'ch > { chars : & 'ch str , }
    };
}

BytesProducer!();