macro_rules! deps {
    () => {
        Generator!();
        Scope!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < A , T > Scope < '_ , 'static , A , T > { # [doc = " yield and get the send para"] # [inline] pub fn yield_ (& mut self , v : T) -> Option < A > { unsafe { self . yield_unsafe (v) } } # [doc = " `yield_from`"] # [doc = " the from generator must has the same type as itself"] pub fn yield_from (& mut self , g : Generator < A , T >) -> Option < A > { unsafe { self . yield_from_unsafe (g) } } }
    };
}

impl_46!()