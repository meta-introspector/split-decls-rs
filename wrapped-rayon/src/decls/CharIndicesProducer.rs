macro_rules! CharIndicesProducer {
    () => {
        struct CharIndicesProducer < 'ch > { index : usize , chars : & 'ch str , }
    };
}

CharIndicesProducer!();