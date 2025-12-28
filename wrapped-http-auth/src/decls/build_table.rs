macro_rules! build_table {
    () => {
        const fn build_table () -> [u8 ; 128] { let mut table = [0u8 ; 128] ; let mut i = 0 ; while i < 128 { let b = i as u8 ; let mut classes = 0 ; if is_tchar (b) { classes |= C_TCHAR ; } if is_qdtext (b) { classes |= C_QDTEXT ; } if is_escapable (b) { classes |= C_ESCAPABLE ; } if is_ows (b) { classes |= C_OWS ; } if is_attr (b) { classes |= C_ATTR ; } table [i] = classes ; i += 1 ; } table }
    };
}

build_table!()