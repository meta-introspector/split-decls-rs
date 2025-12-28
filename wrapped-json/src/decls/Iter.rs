macro_rules! deps {
    () => {
        IterImpl!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        # [doc = " An iterator over a serde_json::Map's entries."] # [derive (Clone , Debug)] pub struct Iter < 'a > { iter : IterImpl < 'a > , }
    };
}

Iter!();