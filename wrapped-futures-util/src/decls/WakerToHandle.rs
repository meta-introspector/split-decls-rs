macro_rules! WakerToHandle {
    () => {
        # [derive (Clone)] struct WakerToHandle < 'a > (& 'a task03 :: Waker) ;
    };
}

WakerToHandle!();