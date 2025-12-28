macro_rules! IdentIsRaw {
    () => {
        # [derive (PartialEq , Encodable , Decodable , Debug , Copy , Clone , HashStable_Generic)] pub enum IdentIsRaw { No , Yes , }
    };
}

IdentIsRaw!()