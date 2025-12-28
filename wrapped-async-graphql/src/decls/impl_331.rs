macro_rules! deps {
    () => {
        Any!();
        Data!();
    };
}

macro_rules! impl_331 {
    () => {
        deps!();
        impl Data { # [doc = " Insert data."] pub fn insert < D : Any + Send + Sync > (& mut self , data : D) { self . 0 . insert (TypeId :: of :: < D > () , Box :: new (data)) ; } pub (crate) fn merge (& mut self , other : Data) { self . 0 . extend (other . 0) ; } }
    };
}

impl_331!();