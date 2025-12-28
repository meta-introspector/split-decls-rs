macro_rules! ForEachConsumer {
    () => {
        struct ForEachConsumer < 'f , F > { op : & 'f F , }
    };
}

ForEachConsumer!();