macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! impl_761 {
    () => {
        deps!();
        impl < G , I > IndexMut < I > for Frozen < '_ , G > where G : IndexMut < I > , { fn index_mut (& mut self , i : I) -> & mut G :: Output { self . 0 . index_mut (i) } }
    };
}

impl_761!()