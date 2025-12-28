macro_rules! EnumerateConsumer {
    () => {
        # [pin_project] struct EnumerateConsumer < C > { # [pin] inner : C , count : usize , }
    };
}

EnumerateConsumer!();