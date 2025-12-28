macro_rules! deps {
    () => {
        Unfold!();
    };
}

macro_rules! impl_484 {
    () => {
        deps!();
        impl < A , St , F > Iterator for Unfold < St , F > where F : FnMut (& mut St) -> Option < A > , { type Item = A ; # [inline] fn next (& mut self) -> Option < Self :: Item > { (self . f) (& mut self . state) } }
    };
}

impl_484!()