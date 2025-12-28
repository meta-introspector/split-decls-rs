macro_rules! MatchIndicesProducer {
    () => {
        struct MatchIndicesProducer < 'ch , 'pat , P : Pattern > { index : usize , chars : & 'ch str , pattern : & 'pat P , }
    };
}

MatchIndicesProducer!()