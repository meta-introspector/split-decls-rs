macro_rules! deps {
    () => {
        FasNodeIndex!();
        FasNode!();
        FasNodeContainer!();
    };
}

macro_rules! impl_367 {
    () => {
        deps!();
        impl Index < FasNodeIndex > for FasNodeContainer { type Output = LinkedListEntry < FasNode , FasNodeIndex > ; fn index (& self , index : FasNodeIndex) -> & Self :: Output { & self . nodes [index . 0] } }
    };
}

impl_367!();