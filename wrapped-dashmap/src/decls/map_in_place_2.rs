macro_rules! deps {
    () => {
        AbortOnPanic!();
    };
}

macro_rules! map_in_place_2 {
    () => {
        deps!();
        pub fn map_in_place_2 < T , U , F : FnOnce (U , T) -> T > ((k , v) : (U , & mut T) , f : F) { unsafe { let promote_panic_to_abort = AbortOnPanic ; ptr :: write (v , f (k , ptr :: read (v))) ; std :: mem :: forget (promote_panic_to_abort) ; } }
    };
}

map_in_place_2!()