macro_rules! Endianness {
    () => {
        # [doc = " An endianness that is selectable at run-time."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum Endianness { # [doc = " Little endian byte order."] Little , # [doc = " Big endian byte order."] Big , }
    };
}

Endianness!();