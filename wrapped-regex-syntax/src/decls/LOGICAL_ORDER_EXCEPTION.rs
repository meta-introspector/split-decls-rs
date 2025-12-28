macro_rules! LOGICAL_ORDER_EXCEPTION {
    () => {
        pub const LOGICAL_ORDER_EXCEPTION : & 'static [(char , char)] = & [('เ' , 'ไ') , ('ເ' , 'ໄ') , ('ᦵ' , 'ᦷ') , ('ᦺ' , 'ᦺ') , ('ꪵ' , 'ꪶ') , ('ꪹ' , 'ꪹ') , ('ꪻ' , 'ꪼ') ,] ;
    };
}

LOGICAL_ORDER_EXCEPTION!()