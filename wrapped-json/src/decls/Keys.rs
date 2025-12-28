macro_rules! deps {
    () => {
        KeysImpl!();
    };
}

macro_rules! Keys {
    () => {
        deps!();
        # [doc = " An iterator over a serde_json::Map's keys."] # [derive (Clone , Debug)] pub struct Keys < 'a > { iter : KeysImpl < 'a > , }
    };
}

Keys!();