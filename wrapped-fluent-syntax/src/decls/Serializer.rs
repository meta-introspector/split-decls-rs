macro_rules! deps {
    () => {
        TextWriter!();
        State!();
        Options!();
    };
}

macro_rules! Serializer {
    () => {
        deps!();
        # [derive (Debug)] pub struct Serializer { writer : TextWriter , options : Options , state : State , }
    };
}

Serializer!()