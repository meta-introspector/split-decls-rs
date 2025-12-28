macro_rules! deps {
    () => {
        LineAnnotation!();
    };
}

macro_rules! overlaps {
    () => {
        deps!();
        fn overlaps (a1 : & LineAnnotation < '_ > , a2 : & LineAnnotation < '_ > , padding : usize) -> bool { num_overlap (a1 . start . display , a1 . end . display + padding , a2 . start . display , a2 . end . display , false ,) }
    };
}

overlaps!()