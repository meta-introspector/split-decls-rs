macro_rules! deps {
    () => {
        Utf8BoundedMap!();
        Utf8Node!();
    };
}

macro_rules! Utf8State {
    () => {
        deps!();
        # [derive (Clone , Debug)] struct Utf8State { compiled : Utf8BoundedMap , uncompiled : Vec < Utf8Node > , }
    };
}

Utf8State!()