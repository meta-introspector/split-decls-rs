macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl Label { pub fn new (label : String) -> Label { always ! (label . starts_with (char :: is_uppercase) && ! label . ends_with ('.')) ; Label (label) } }
    };
}

impl_87!();