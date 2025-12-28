macro_rules! deps {
    () => {
        DoubleEndedFallibleIterator!();
    };
}

macro_rules! _is_object_safe {
    () => {
        deps!();
        fn _is_object_safe (_ : & dyn DoubleEndedFallibleIterator < Item = () , Error = () >) { }
    };
}

_is_object_safe!();