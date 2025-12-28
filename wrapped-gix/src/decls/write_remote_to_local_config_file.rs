macro_rules! deps {
    () => {
        Error!();
        WriteMode!();
        Remote!();
    };
}

macro_rules! write_remote_to_local_config_file {
    () => {
        deps!();
        # [allow (clippy :: result_large_err)] pub fn write_remote_to_local_config_file (remote : & mut crate :: Remote < '_ > , remote_name : BString ,) -> Result < gix_config :: File < 'static > , Error > { let mut config = gix_config :: File :: new (local_config_meta (remote . repo)) ; remote . save_as_to (remote_name , & mut config) ? ; write_to_local_config (& config , WriteMode :: Append) ? ; Ok (config) }
    };
}

write_remote_to_local_config_file!()