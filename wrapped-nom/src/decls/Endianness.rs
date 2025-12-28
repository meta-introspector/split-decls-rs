macro_rules! Endianness {
    () => {
        # [doc = " Configurable endianness"] # [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum Endianness { # [doc = " Big endian"] Big , # [doc = " Little endian"] Little , # [doc = " Will match the host's endianness"] Native , }
    };
}

Endianness!()