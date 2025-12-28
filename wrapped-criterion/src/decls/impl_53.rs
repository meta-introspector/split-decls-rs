macro_rules! deps {
    () => {
        MessageError!();
        Error!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl std :: error :: Error for MessageError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { MessageError :: Deserialization (err) => Some (err) , MessageError :: Serialization (err) => Some (err) , MessageError :: Io (err) => Some (err) , } } }
    };
}

impl_53!()