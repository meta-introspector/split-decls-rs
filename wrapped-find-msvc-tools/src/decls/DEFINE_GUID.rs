macro_rules! deps {
    () => {
        GUID!();
    };
}

macro_rules! DEFINE_GUID {
    () => {
        deps!();
        macro_rules ! DEFINE_GUID { ($ name : ident , $ l : expr , $ w1 : expr , $ w2 : expr , $ b1 : expr , $ b2 : expr , $ b3 : expr , $ b4 : expr , $ b5 : expr , $ b6 : expr , $ b7 : expr , $ b8 : expr) => { pub const $ name : $ crate :: winapi :: GUID = $ crate :: winapi :: GUID { data1 : $ l , data2 : $ w1 , data3 : $ w2 , data4 : [$ b1 , $ b2 , $ b3 , $ b4 , $ b5 , $ b6 , $ b7 , $ b8] , } ; } ; }
    };
}

DEFINE_GUID!();