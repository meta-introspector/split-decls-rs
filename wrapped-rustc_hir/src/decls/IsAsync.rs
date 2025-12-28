macro_rules! IsAsync {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Encodable , Decodable , Debug , HashStable_Generic)] pub enum IsAsync { Async (Span) , NotAsync , }
    };
}

IsAsync!()