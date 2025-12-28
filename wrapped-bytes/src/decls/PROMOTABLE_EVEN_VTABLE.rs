macro_rules! deps {
    () => {
        Vtable!();
    };
}

macro_rules! PROMOTABLE_EVEN_VTABLE {
    () => {
        deps!();
        static PROMOTABLE_EVEN_VTABLE : Vtable = Vtable { clone : promotable_even_clone , into_vec : promotable_even_to_vec , into_mut : promotable_even_to_mut , is_unique : promotable_is_unique , drop : promotable_even_drop , } ;
    };
}

PROMOTABLE_EVEN_VTABLE!();