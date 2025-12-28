macro_rules! UpdateProducer {
    () => {
        struct UpdateProducer < 'f , P , F > { base : P , update_op : & 'f F , }
    };
}

UpdateProducer!()