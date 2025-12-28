macro_rules! deps {
    () => {
        Candidate!();
        Match!();
    };
}

macro_rules! impl_371 {
    () => {
        deps!();
        impl Candidate { # [doc = " Convert this candidate into an option. This is useful when callers"] # [doc = " do not distinguish between true positives and false positives (i.e.,"] # [doc = " the caller must always confirm the match)."] pub fn into_option (self) -> Option < usize > { match self { Candidate :: None => None , Candidate :: Match (ref m) => Some (m . start ()) , Candidate :: PossibleStartOfMatch (start) => Some (start) , } } }
    };
}

impl_371!()