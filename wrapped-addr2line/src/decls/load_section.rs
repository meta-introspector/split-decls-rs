macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! load_section {
    () => {
        deps!();
        fn load_section < 'input , Endian : gimli :: Endianity > (name : Option < & 'static str > , file : & object :: File < 'input > , endian : Endian , arena_data : & 'input Arena < Vec < u8 > > ,) -> Result < gimli :: EndianSlice < 'input , Endian > > { let data = match name . and_then (| name | file . section_by_name (name)) { Some (section) => match section . uncompressed_data () ? { Cow :: Borrowed (b) => b , Cow :: Owned (b) => arena_data . alloc (b) , } , None => & [] , } ; Ok (gimli :: EndianSlice :: new (data , endian)) }
    };
}

load_section!();