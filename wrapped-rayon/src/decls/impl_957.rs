macro_rules! deps {
    () => {
        UnzipReducer!();
        Reducer!();
    };
}

macro_rules! impl_957 {
    () => {
        deps!();
        impl < A , B , RA , RB > Reducer < (A , B) > for UnzipReducer < RA , RB > where RA : Reducer < A > , RB : Reducer < B > , { fn reduce (self , left : (A , B) , right : (A , B)) -> (A , B) { (self . left . reduce (left . 0 , right . 0) , self . right . reduce (left . 1 , right . 1) ,) } }
    };
}

impl_957!();