macro_rules! deps {
    () => {
        ValuesMutImpl!();
    };
}

macro_rules! ValuesMut {
    () => {
        deps!();
        # [doc = " A mutable iterator over a serde_json::Map's values."] # [derive (Debug)] pub struct ValuesMut < 'a > { iter : ValuesMutImpl < 'a > , }
    };
}

ValuesMut!();