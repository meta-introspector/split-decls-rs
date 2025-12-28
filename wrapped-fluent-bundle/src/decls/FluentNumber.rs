macro_rules! deps {
    () => {
        FluentNumberOptions!();
    };
}

macro_rules! FluentNumber {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq)] pub struct FluentNumber { pub value : f64 , pub options : FluentNumberOptions , }
    };
}

FluentNumber!();