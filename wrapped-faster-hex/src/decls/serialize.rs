macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! serialize {
    () => {
        deps!();
        # [doc = " Serde: Serialize with 0x-prefix and ignore case"] pub fn serialize < S , T > (data : T , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , T : AsRef < [u8] > , { withpfx_ignorecase :: serialize (data , serializer) }
    };
}

serialize!();