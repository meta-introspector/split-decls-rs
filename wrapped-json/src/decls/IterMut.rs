macro_rules! deps {
    () => {
        IterMutImpl!();
    };
}

macro_rules! IterMut {
    () => {
        deps!();
        # [doc = " A mutable iterator over a serde_json::Map's entries."] # [derive (Debug)] pub struct IterMut < 'a > { iter : IterMutImpl < 'a > , }
    };
}

IterMut!();