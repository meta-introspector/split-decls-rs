macro_rules! Defaultness {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Debug , Encodable , Decodable , HashStable_Generic)] pub enum Defaultness { Default { has_value : bool } , Final , }
    };
}

Defaultness!();