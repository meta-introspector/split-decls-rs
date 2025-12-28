macro_rules! deps {
    () => {
        Vtable!();
    };
}

macro_rules! STATIC_VTABLE {
    () => {
        deps!();
        const STATIC_VTABLE : Vtable = Vtable { clone : static_clone , into_vec : static_to_vec , into_mut : static_to_mut , is_unique : static_is_unique , drop : static_drop , } ;
    };
}

STATIC_VTABLE!()