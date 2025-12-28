macro_rules! deps {
    () => {
        CollectSeqWithLength!();
    };
}

macro_rules! impl_1056 {
    () => {
        deps!();
        impl < S > CollectSeqWithLength for S where S : Serializer { }
    };
}

impl_1056!();