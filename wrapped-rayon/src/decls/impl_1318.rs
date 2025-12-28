macro_rules! deps {
    () => {
        SplitProducer!();
        SplitTerminatorProducer!();
    };
}

macro_rules! impl_1318 {
    () => {
        deps!();
        impl < 'ch , 'sep , P : Pattern + 'sep > SplitTerminatorProducer < 'ch , 'sep , P > { fn new (chars : & 'ch str , terminator : & 'sep P) -> Self { SplitTerminatorProducer { splitter : SplitProducer :: new (chars , terminator) , skip_last : chars . is_empty () || terminator . is_suffix_of (chars) , } } }
    };
}

impl_1318!()