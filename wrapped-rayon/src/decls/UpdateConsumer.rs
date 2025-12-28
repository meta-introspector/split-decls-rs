macro_rules! UpdateConsumer {
    () => {
        struct UpdateConsumer < 'f , C , F > { base : C , update_op : & 'f F , }
    };
}

UpdateConsumer!();