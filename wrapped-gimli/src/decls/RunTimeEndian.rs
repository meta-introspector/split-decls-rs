macro_rules! RunTimeEndian {
    () => {
        # [doc = " Byte order that is selectable at runtime."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum RunTimeEndian { # [doc = " Little endian byte order."] Little , # [doc = " Big endian byte order."] Big , }
    };
}

RunTimeEndian!()