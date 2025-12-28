macro_rules! Kind {
    () => {
        # [derive (Clone , Copy , Eq , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub enum Kind { Anything , Integer , Pointer , Half , Float , Double , Unknown , }
    };
}

Kind!()