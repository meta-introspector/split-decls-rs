macro_rules! MatchesProducer {
    () => {
        struct MatchesProducer < 'ch , 'pat , P : Pattern > { chars : & 'ch str , pattern : & 'pat P , }
    };
}

MatchesProducer!()