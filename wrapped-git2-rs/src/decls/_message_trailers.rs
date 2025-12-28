macro_rules! deps {
    () => {
        MessageTrailers!();
        Error!();
    };
}

macro_rules! _message_trailers {
    () => {
        deps!();
        fn _message_trailers (message : CString) -> Result < MessageTrailers , Error > { let ret = MessageTrailers :: new () ; unsafe { try_call ! (raw :: git_message_trailers (ret . raw () , message)) ; } Ok (ret) }
    };
}

_message_trailers!();