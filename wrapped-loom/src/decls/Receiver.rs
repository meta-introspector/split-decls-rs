macro_rules! deps {
    () => {
        Channel!();
        Arc!();
    };
}

macro_rules! Receiver {
    () => {
        deps!();
        # [derive (Debug)] # [doc = " Mock implementation of `std::sync::mpsc::Receiver`."] pub struct Receiver < T > { object : std :: sync :: Arc < rt :: Channel > , receiver : std :: sync :: mpsc :: Receiver < T > , }
    };
}

Receiver!();