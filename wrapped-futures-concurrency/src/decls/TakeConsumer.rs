macro_rules! TakeConsumer {
    () => {
        # [pin_project] struct TakeConsumer < C > { # [pin] inner : C , count : usize , limit : usize , }
    };
}

TakeConsumer!()