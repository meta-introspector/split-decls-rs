macro_rules! ObjectAccessor {
    () => {
        # [doc = " A object accessor"] pub struct ObjectAccessor < 'a > (pub (crate) Cow < 'a , IndexMap < Name , Value > >) ;
    };
}

ObjectAccessor!()