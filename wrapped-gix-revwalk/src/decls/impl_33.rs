macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < K , T > Clone for Item < K , T > where K : Clone , T : Clone , { fn clone (& self) -> Self { Item { key : self . key . clone () , value : self . value . clone () , } } }
    };
}

impl_33!()