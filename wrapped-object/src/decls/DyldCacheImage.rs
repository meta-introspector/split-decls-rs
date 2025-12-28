macro_rules! deps {
    () => {
        Endianness!();
        Endian!();
        ReadRef!();
        DyldCacheImageInfo!();
        DyldCache!();
    };
}

macro_rules! DyldCacheImage {
    () => {
        deps!();
        # [doc = " One image (dylib) from inside the dyld shared cache."] # [derive (Debug)] pub struct DyldCacheImage < 'data , 'cache , E = Endianness , R = & 'data [u8] > where E : Endian , R : ReadRef < 'data > , { pub (crate) cache : & 'cache DyldCache < 'data , E , R > , image_info : & 'data macho :: DyldCacheImageInfo < E > , }
    };
}

DyldCacheImage!();