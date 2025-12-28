macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! BernoulliError {
    () => {
        deps!();
        # [doc = " Error type returned from [`Bernoulli::new`]."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum BernoulliError { # [doc = " `p < 0` or `p > 1`."] InvalidProbability , }
    };
}

BernoulliError!()