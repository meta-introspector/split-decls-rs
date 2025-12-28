macro_rules! deps {
    () => {
        Compat!();
    };
}

macro_rules! Executor01Future {
    () => {
        deps!();
        # [doc = " A future that can run on a futures 0.1"] # [doc = " [`Executor`](futures_01::future::Executor)."] pub type Executor01Future = Compat < UnitError < FutureObj < 'static , () > > > ;
    };
}

Executor01Future!()