macro_rules! deps {
    () => {
        Bencher!();
        AsyncExecutor!();
        Measurement!();
        WallTime!();
    };
}

macro_rules! AsyncBencher {
    () => {
        deps!();
        # [doc = " Async/await variant of [`Bencher`]."] # [cfg (feature = "async")] pub struct AsyncBencher < 'a , 'b , A : AsyncExecutor , M : Measurement = WallTime > { b : & 'b mut Bencher < 'a , M > , runner : A , }
    };
}

AsyncBencher!();