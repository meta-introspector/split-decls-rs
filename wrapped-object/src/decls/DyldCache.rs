macro_rules! deps {
    () => {
        ReadRef!();
        Endianness!();
        Architecture!();
        Endian!();
        DyldFile!();
        DyldCacheImageInfo!();
    };
}

macro_rules! DyldCache {
    () => {
        deps!();
        # [doc = " A parsed representation of the dyld shared cache."] # [derive (Debug)] pub struct DyldCache < 'data , E = Endianness , R = & 'data [u8] > where E : Endian , R : ReadRef < 'data > , { endian : E , data : R , # [doc = " The first entry is the main cache file, and the rest are subcaches."] files : Vec < DyldFile < 'data , E , R > > , images : & 'data [macho :: DyldCacheImageInfo < E >] , arch : Architecture , }
    };
}

DyldCache!()