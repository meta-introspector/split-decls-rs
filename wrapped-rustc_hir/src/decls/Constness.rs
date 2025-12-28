macro_rules! Constness {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Debug , Encodable , Decodable , HashStable_Generic)] pub enum Constness { Const , NotConst , }
    };
}

Constness!();