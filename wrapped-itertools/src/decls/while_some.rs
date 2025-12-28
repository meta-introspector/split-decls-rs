macro_rules! deps {
    () => {
        WhileSome!();
    };
}

macro_rules! while_some {
    () => {
        deps!();
        # [doc = " Create a new `WhileSome<I>`."] pub fn while_some < I > (iter : I) -> WhileSome < I > { WhileSome { iter } }
    };
}

while_some!()