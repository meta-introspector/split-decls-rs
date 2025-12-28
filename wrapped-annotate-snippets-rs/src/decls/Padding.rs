macro_rules! deps {
    () => {
        Element!();
        Group!();
    };
}

macro_rules! Padding {
    () => {
        deps!();
        # [doc = " A whitespace [`Element`] in a [`Group`]"] # [derive (Clone , Debug)] pub struct Padding ;
    };
}

Padding!()