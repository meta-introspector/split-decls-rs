macro_rules! CharsProducer {
    () => {
        struct CharsProducer < 'ch > { chars : & 'ch str , }
    };
}

CharsProducer!();