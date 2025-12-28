macro_rules! maybe_mmap_file {
    () => {
        # [cfg (feature = "mmap")] pub (crate) fn maybe_mmap_file (file : & std :: fs :: File) -> std :: io :: Result < Option < memmap2 :: Mmap > > { let metadata = file . metadata () ? ; let file_size = metadata . len () ; if ! metadata . is_file () { Ok (None) } else if file_size < 16 * 1024 { Ok (None) } else { let map = unsafe { memmap2 :: Mmap :: map (file) ? } ; Ok (Some (map)) } }
    };
}

maybe_mmap_file!();