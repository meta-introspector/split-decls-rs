macro_rules! deps {
    () => {
        ReadRef!();
        Result!();
    };
}

macro_rules! parse_bsd_extended_name {
    () => {
        deps!();
        # [doc = " Digits are a decimal length of the extended name, which is contained"] # [doc = " in `data` at `offset`."] # [doc = " Modifies `offset` and `size` to start after the extended name."] fn parse_bsd_extended_name < 'data , R : ReadRef < 'data > > (digits : & [u8] , data : R , offset : & mut u64 , size : & mut u64 ,) -> Result < & 'data [u8] , () > { let len = parse_u64_digits (digits , 10) . ok_or (()) ? ; * size = size . checked_sub (len) . ok_or (()) ? ; let name_data = data . read_bytes (offset , len) ? ; let name = match memchr :: memchr (b'\0' , name_data) { Some (len) => & name_data [.. len] , None => name_data , } ; Ok (name) }
    };
}

parse_bsd_extended_name!()