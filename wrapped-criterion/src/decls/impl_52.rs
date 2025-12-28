macro_rules! deps {
    () => {
        Result!();
        MessageError!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl std :: fmt :: Display for MessageError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { MessageError :: Deserialization (error) => write ! (f , "Failed to deserialize message to Criterion.rs benchmark:\n{}" , error) , MessageError :: Serialization (error) => write ! (f , "Failed to serialize message to Criterion.rs benchmark:\n{}" , error) , MessageError :: Io (error) => write ! (f , "Failed to read or write message to Criterion.rs benchmark:\n{}" , error) , } } }
    };
}

impl_52!();