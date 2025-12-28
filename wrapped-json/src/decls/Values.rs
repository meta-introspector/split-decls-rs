macro_rules! deps {
    () => {
        ValuesImpl!();
    };
}

macro_rules! Values {
    () => {
        deps!();
        # [doc = " An iterator over a serde_json::Map's values."] # [derive (Clone , Debug)] pub struct Values < 'a > { iter : ValuesImpl < 'a > , }
    };
}

Values!();