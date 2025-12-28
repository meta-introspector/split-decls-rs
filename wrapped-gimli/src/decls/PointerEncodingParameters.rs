macro_rules! deps {
    () => {
        SectionBaseAddresses!();
        Reader!();
    };
}

macro_rules! PointerEncodingParameters {
    () => {
        deps!();
        # [derive (Clone , Debug)] struct PointerEncodingParameters < 'a , R : Reader > { bases : & 'a SectionBaseAddresses , func_base : Option < u64 > , address_size : u8 , section : & 'a R , }
    };
}

PointerEncodingParameters!()