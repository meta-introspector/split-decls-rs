macro_rules! deps {
    () => {
        DecompressorOxide!();
    };
}

macro_rules! start_static_table {
    () => {
        deps!();
        fn start_static_table (r : & mut DecompressorOxide) { r . table_sizes [LITLEN_TABLE] = 288 ; r . table_sizes [DIST_TABLE] = 32 ; r . code_size_literal [0 .. 144] . fill (8) ; r . code_size_literal [144 .. 256] . fill (9) ; r . code_size_literal [256 .. 280] . fill (7) ; r . code_size_literal [280 .. 288] . fill (8) ; r . code_size_dist [0 .. 32] . fill (5) ; }
    };
}

start_static_table!()