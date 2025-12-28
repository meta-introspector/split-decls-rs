macro_rules! deps {
    () => {
        FasNodeIndex!();
        FasNodeContainer!();
    };
}

macro_rules! impl_368 {
    () => {
        deps!();
        impl IndexMut < FasNodeIndex > for FasNodeContainer { fn index_mut (& mut self , index : FasNodeIndex) -> & mut Self :: Output { & mut self . nodes [index . 0] } }
    };
}

impl_368!()