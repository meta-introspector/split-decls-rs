macro_rules! deps {
    () => {
        FlatPairs!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < 'i , R : Clone > Clone for FlatPairs < 'i , R > { fn clone (& self) -> FlatPairs < 'i , R > { FlatPairs { queue : Rc :: clone (& self . queue) , input : self . input , line_index : Rc :: clone (& self . line_index) , start : self . start , end : self . end , } } }
    };
}

impl_29!()