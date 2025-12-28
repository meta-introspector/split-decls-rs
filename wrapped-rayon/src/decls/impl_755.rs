macro_rules! deps {
    () => {
        Reducer!();
        PanicFuseReducer!();
    };
}

macro_rules! impl_755 {
    () => {
        deps!();
        impl < 'a , T , C > Reducer < T > for PanicFuseReducer < 'a , C > where C : Reducer < T > , { fn reduce (self , left : T , right : T) -> T { self . base . reduce (left , right) } }
    };
}

impl_755!();