macro_rules! InspectProducer {
    () => {
        struct InspectProducer < 'f , P , F > { base : P , inspect_op : & 'f F , }
    };
}

InspectProducer!();