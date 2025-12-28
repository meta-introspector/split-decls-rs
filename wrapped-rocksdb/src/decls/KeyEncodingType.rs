macro_rules! deps {
    () => {
        PlainTableFactoryOptions!();
    };
}

macro_rules! KeyEncodingType {
    () => {
        deps!();
        # [doc = " Used in [`PlainTableFactoryOptions`]."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Default)] pub enum KeyEncodingType { # [doc = " Always write full keys."] # [default] Plain = 0 , # [doc = " Find opportunities to write the same prefix for multiple rows."] Prefix = 1 , }
    };
}

KeyEncodingType!()