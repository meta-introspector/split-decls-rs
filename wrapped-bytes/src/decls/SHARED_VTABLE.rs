macro_rules! deps {
    () => {
        Vtable!();
    };
}

macro_rules! SHARED_VTABLE {
    () => {
        deps!();
        static SHARED_VTABLE : Vtable = Vtable { clone : shared_v_clone , into_vec : shared_v_to_vec , into_mut : shared_v_to_mut , is_unique : shared_v_is_unique , drop : shared_v_drop , } ;
    };
}

SHARED_VTABLE!();