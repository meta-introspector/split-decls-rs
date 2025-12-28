macro_rules! deps {
    () => {
        RepeatN!();
    };
}

macro_rules! impl_467 {
    () => {
        deps!();
        impl < A > DoubleEndedIterator for RepeatN < A > where A : Clone , { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { self . next () } # [inline] fn rfold < B , F > (self , init : B , f : F) -> B where F : FnMut (B , Self :: Item) -> B , { self . fold (init , f) } }
    };
}

impl_467!()