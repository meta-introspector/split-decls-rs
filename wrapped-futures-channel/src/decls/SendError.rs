macro_rules! deps {
    () => {
        SendErrorKind!();
    };
}

macro_rules! SendError {
    () => {
        deps!();
        # [doc = " The error type for [`Sender`s](Sender) used as `Sink`s."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct SendError { kind : SendErrorKind , }
    };
}

SendError!()