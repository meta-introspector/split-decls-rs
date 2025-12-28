macro_rules! deps {
    () => {
        TupledDistributionsBuilder!();
        Distributions!();
        TupledDistributions!();
    };
}

macro_rules! Tuple {
    () => {
        deps!();
        # [doc = " Any tuple: `(A, B, ..)`"] pub trait Tuple : Sized { # [doc = " A tuple of distributions associated with this tuple"] type Distributions : TupledDistributions < Item = Self > ; # [doc = " A tuple of vectors associated with this tuple"] type Builder : TupledDistributionsBuilder < Item = Self > ; }
    };
}

Tuple!()