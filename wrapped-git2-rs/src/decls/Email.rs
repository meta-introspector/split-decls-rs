macro_rules! deps {
    () => {
        Buf!();
    };
}

macro_rules! Email {
    () => {
        deps!();
        # [doc = " A structure to represent patch in mbox format for sending via email"] pub struct Email { buf : Buf , }
    };
}

Email!()