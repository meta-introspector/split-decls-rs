macro_rules! deps {
    () => {
        KeyFunction!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl < A , K , F > KeyFunction < A > for F where F : FnMut (A) -> K + ? Sized , { type Key = K ; # [inline] fn call_mut (& mut self , arg : A) -> Self :: Key { (* self) (arg) } }
    };
}

impl_252!()