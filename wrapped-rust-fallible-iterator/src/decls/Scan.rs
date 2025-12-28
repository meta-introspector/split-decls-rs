macro_rules! Scan {
    () => {
        # [doc = " An iterator which applies a stateful closure."] # [derive (Clone , Debug)] pub struct Scan < I , St , F > { it : I , f : F , state : St , }
    };
}

Scan!()