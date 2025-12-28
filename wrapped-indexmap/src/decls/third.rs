macro_rules! third {
    () => {
        pub (crate) fn third < A , B , C > (t : (A , B , C)) -> C { t . 2 }
    };
}

third!()