macro_rules! LimitConsumer {
    () => {
        # [pin_project] struct LimitConsumer < C > { # [pin] inner : C , }
    };
}

LimitConsumer!()