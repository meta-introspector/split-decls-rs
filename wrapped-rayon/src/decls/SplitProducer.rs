macro_rules! SplitProducer {
    () => {
        struct SplitProducer < 'a , D , S > { data : D , splitter : & 'a S , }
    };
}

SplitProducer!();