macro_rules! deps {
    () => {
        Element!();
        Level!();
        Group!();
    };
}

macro_rules! Message {
    () => {
        deps!();
        # [doc = " A text [`Element`] in a [`Group`]"] # [doc = ""] # [doc = " See [`Level::message`] to create this."] # [derive (Clone , Debug)] pub struct Message < 'a > { pub (crate) level : Level < 'a > , pub (crate) text : Cow < 'a , str > , }
    };
}

Message!();