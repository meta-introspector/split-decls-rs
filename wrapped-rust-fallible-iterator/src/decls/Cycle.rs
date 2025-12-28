macro_rules! Cycle {
    () => {
        # [doc = " An iterator which cycles another endlessly."] # [derive (Clone , Debug)] pub struct Cycle < I > { it : I , cur : I , }
    };
}

Cycle!();