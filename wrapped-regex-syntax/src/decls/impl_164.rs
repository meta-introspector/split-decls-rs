macro_rules! deps {
    () => {
        Literal!();
        Seq!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl FromIterator < Literal > for Seq { fn from_iter < T : IntoIterator < Item = Literal > > (it : T) -> Seq { let mut seq = Seq :: empty () ; for literal in it { seq . push (literal) ; } seq } }
    };
}

impl_164!()