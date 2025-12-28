macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Consumed {
    () => {
        deps!();
        # [doc = " Parser implementation for [consumed]"] pub struct Consumed < F > { parser : F , }
    };
}

Consumed!()