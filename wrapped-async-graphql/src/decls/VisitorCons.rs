macro_rules! VisitorCons {
    () => {
        pub (crate) struct VisitorCons < A , B > (A , B) ;
    };
}

VisitorCons!();