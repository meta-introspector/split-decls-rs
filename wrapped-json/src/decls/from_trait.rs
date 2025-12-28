macro_rules! deps {
    () => {
        Deserializer!();
        Result!();
        Read!();
    };
}

macro_rules! from_trait {
    () => {
        deps!();
        fn from_trait < 'de , R , T > (read : R) -> Result < T > where R : Read < 'de > , T : de :: Deserialize < 'de > , { let mut de = Deserializer :: new (read) ; let value = tri ! (de :: Deserialize :: deserialize (& mut de)) ; tri ! (de . end ()) ; Ok (value) }
    };
}

from_trait!()