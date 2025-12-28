macro_rules! deps {
    () => {
        ExactlyOneError!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        impl < I > Debug for ExactlyOneError < I > where I : Iterator + Debug , I :: Item : Debug , { fn fmt (& self , f : & mut Formatter) -> FmtResult { let mut dbg = f . debug_struct ("ExactlyOneError") ; match & self . first_two { Some (Either :: Left ([first , second])) => { dbg . field ("first" , first) . field ("second" , second) ; } Some (Either :: Right (second)) => { dbg . field ("second" , second) ; } None => { } } dbg . field ("inner" , & self . inner) . finish () } }
    };
}

impl_222!();