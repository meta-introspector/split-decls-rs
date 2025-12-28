macro_rules! deps {
    () => {
        PubStuffEntry!();
        Reader!();
    };
}

macro_rules! PubStuffParser {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct PubStuffParser < R , Entry > where R : Reader , Entry : PubStuffEntry < R > , { phantom : PhantomData < (R , Entry) > , }
    };
}

PubStuffParser!()