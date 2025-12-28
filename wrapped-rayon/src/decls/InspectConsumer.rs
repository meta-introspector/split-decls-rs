macro_rules! InspectConsumer {
    () => {
        struct InspectConsumer < 'f , C , F > { base : C , inspect_op : & 'f F , }
    };
}

InspectConsumer!();