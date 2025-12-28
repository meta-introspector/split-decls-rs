macro_rules! TestingRandom {
    () => {
        # [cfg (test)] # [cfg (feature = "safe_api")] pub trait TestingRandom { # [doc = " Randomly generate self."] fn gen () -> Self ; }
    };
}

TestingRandom!()