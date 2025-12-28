macro_rules! ValueAccessor {
    () => {
        # [doc = " A value accessor"] pub struct ValueAccessor < 'a > (& 'a Value) ;
    };
}

ValueAccessor!()