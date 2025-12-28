macro_rules! SenderTask {
    () => {
        struct SenderTask { task : Option < Waker > , is_parked : bool , }
    };
}

SenderTask!()