macro_rules! deps {
    () => {
        Epoch!();
        Bag!();
    };
}

macro_rules! SealedBag {
    () => {
        deps!();
        # [doc = " A pair of an epoch and a bag."] # [derive (Default , Debug)] struct SealedBag { epoch : Epoch , _bag : Bag , }
    };
}

SealedBag!()