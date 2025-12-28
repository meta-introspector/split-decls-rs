macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! MessageError {
    () => {
        deps!();
        # [derive (Debug)] pub enum MessageError { Deserialization (ciborium :: de :: Error < std :: io :: Error >) , Serialization (ciborium :: ser :: Error < std :: io :: Error >) , Io (std :: io :: Error) , }
    };
}

MessageError!();