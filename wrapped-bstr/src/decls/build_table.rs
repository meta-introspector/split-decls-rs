macro_rules! build_table {
    () => {
        # [inline] fn build_table (byteset : & [u8]) -> [u8 ; 256] { let mut table = [0u8 ; 256] ; for & b in byteset { table [b as usize] = 1 ; } table }
    };
}

build_table!();