macro_rules! deps {
    () => {
        CheckCase!();
    };
}

macro_rules! init_unhex4_array {
    () => {
        deps!();
        const fn init_unhex4_array (check_case : CheckCase) -> [u8 ; 256] { let unhex_arr = init_unhex_array (check_case) ; let mut unhex4_arr = [NIL ; 256] ; let mut i = 0 ; while i < 256 { if unhex_arr [i] != NIL { unhex4_arr [i] = unhex_arr [i] << 4 ; } i += 1 ; } unhex4_arr }
    };
}

init_unhex4_array!()