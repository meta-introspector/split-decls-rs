macro_rules! deps {
    () => {
        Utf8Node!();
        Utf8BoundedMap!();
    };
}

macro_rules! Utf8State {
    () => {
        deps!();
        # [derive (Clone , Debug)] struct Utf8State { compiled : Utf8BoundedMap , uncompiled : Vec < Utf8Node > , }
    };
}

Utf8State!();