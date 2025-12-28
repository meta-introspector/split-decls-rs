macro_rules! deps {
    () => {
        Options!();
        TextWriter!();
        State!();
    };
}

macro_rules! Serializer {
    () => {
        deps!();
        # [derive (Debug)] pub struct Serializer { writer : TextWriter , options : Options , state : State , }
    };
}

Serializer!();