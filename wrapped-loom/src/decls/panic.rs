macro_rules! deps {
    () => {
        PanicBuilder!();
    };
}

macro_rules! panic {
    () => {
        deps!();
        pub (super) fn panic (msg : impl ToString) -> PanicBuilder { PanicBuilder { msg : msg . to_string () , locations : Vec :: new () , } }
    };
}

panic!()