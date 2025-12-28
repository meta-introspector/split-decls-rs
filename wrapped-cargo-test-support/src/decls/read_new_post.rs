macro_rules! read_new_post {
    () => {
        # [track_caller] fn read_new_post (new_path : & Path) -> (Vec < u8 > , Vec < u8 >) { let mut f = File :: open (new_path) . unwrap () ; let json_sz = read_le_u32 (& mut f) . expect ("read json length") ; let mut json_bytes = vec ! [0 ; json_sz as usize] ; f . read_exact (& mut json_bytes) . expect ("read JSON data") ; let crate_sz = read_le_u32 (& mut f) . expect ("read crate length") ; let mut krate_bytes = vec ! [0 ; crate_sz as usize] ; f . read_exact (& mut krate_bytes) . expect ("read crate data") ; let current = f . seek (SeekFrom :: Current (0)) . unwrap () ; assert_eq ! (f . seek (SeekFrom :: End (0)) . unwrap () , current) ; (json_bytes , krate_bytes) }
    };
}

read_new_post!();