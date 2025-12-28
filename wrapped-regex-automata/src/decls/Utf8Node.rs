macro_rules! deps {
    () => {
        Utf8LastTransition!();
        Transition!();
    };
}

macro_rules! Utf8Node {
    () => {
        deps!();
        # [derive (Clone , Debug)] struct Utf8Node { trans : Vec < Transition > , last : Option < Utf8LastTransition > , }
    };
}

Utf8Node!();