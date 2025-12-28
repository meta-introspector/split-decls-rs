macro_rules! deps {
    () => {
        Arc!();
        Channel!();
    };
}

macro_rules! Sender {
    () => {
        deps!();
        # [derive (Debug)] # [doc = " Mock implementation of `std::sync::mpsc::Sender`."] pub struct Sender < T > { object : std :: sync :: Arc < rt :: Channel > , sender : std :: sync :: mpsc :: Sender < T > , }
    };
}

Sender!()