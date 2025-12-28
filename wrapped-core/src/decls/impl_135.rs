macro_rules! deps {
    () => {
        GUID!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl core :: fmt :: Debug for GUID { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "{:08X?}-{:04X?}-{:04X?}-{:02X?}{:02X?}-{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}" , self . data1 , self . data2 , self . data3 , self . data4 [0] , self . data4 [1] , self . data4 [2] , self . data4 [3] , self . data4 [4] , self . data4 [5] , self . data4 [6] , self . data4 [7]) } }
    };
}

impl_135!();