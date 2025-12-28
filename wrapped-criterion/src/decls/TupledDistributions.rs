macro_rules! deps {
    () => {
        Tuple!();
        Distributions!();
    };
}

macro_rules! TupledDistributions {
    () => {
        deps!();
        # [doc = " A tuple of distributions: `(Distribution<A>, Distribution<B>, ..)`"] pub trait TupledDistributions : Sized { # [doc = " A tuple that can be pushed/inserted into the tupled distributions"] type Item : Tuple < Distributions = Self > ; }
    };
}

TupledDistributions!();