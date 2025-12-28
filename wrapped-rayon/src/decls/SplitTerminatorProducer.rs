macro_rules! deps {
    () => {
        SplitProducer!();
    };
}

macro_rules! SplitTerminatorProducer {
    () => {
        deps!();
        struct SplitTerminatorProducer < 'ch , 'sep , P : Pattern > { splitter : SplitProducer < 'sep , P , & 'ch str > , skip_last : bool , }
    };
}

SplitTerminatorProducer!();