macro_rules! demangle_stream {
    () => {
        # [doc = " Process a stream of data from `input` into the provided `output`, demangling any symbols found"] # [doc = " within."] # [doc = ""] # [doc = " Note that the underlying implementation will perform many relatively small writes to the"] # [doc = " output. If the output is expensive to write to (e.g., requires syscalls), consider using"] # [doc = " `std::io::BufWriter`."] # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] pub fn demangle_stream < R : std :: io :: BufRead , W : std :: io :: Write > (input : & mut R , output : & mut W , include_hash : bool ,) -> std :: io :: Result < () > { let mut buf = std :: string :: String :: new () ; while input . read_line (& mut buf) ? > 0 { demangle_line (& buf , output , include_hash) ? ; buf . clear () ; } Ok (()) }
    };
}

demangle_stream!();