macro_rules! deps {
    () => {
        Vtable!();
    };
}

macro_rules! PROMOTABLE_ODD_VTABLE {
    () => {
        deps!();
        static PROMOTABLE_ODD_VTABLE : Vtable = Vtable { clone : promotable_odd_clone , into_vec : promotable_odd_to_vec , into_mut : promotable_odd_to_mut , is_unique : promotable_is_unique , drop : promotable_odd_drop , } ;
    };
}

PROMOTABLE_ODD_VTABLE!();