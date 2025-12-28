macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! ValuesMut {
    () => {
        deps!();
        pub struct ValuesMut < 'a , K , V > { inner : IterMut < 'a , K , V > , }
    };
}

ValuesMut!()