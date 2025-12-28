macro_rules! deps {
    () => {
        EndianReader!();
    };
}

macro_rules! EndianArcSlice {
    () => {
        deps!();
        # [doc = " An atomically reference counted, thread-safe slice of bytes and associated"] # [doc = " endianity."] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(feature = \"std\")] {"] # [doc = " use std::sync::Arc;"] # [doc = ""] # [doc = " let buf = Arc::from(&[1, 2, 3, 4][..]);"] # [doc = " let reader = gimli::EndianArcSlice::new(buf, gimli::NativeEndian);"] # [doc = " # let _ = reader;"] # [doc = " # }"] # [doc = " ```"] pub type EndianArcSlice < Endian > = EndianReader < Endian , Arc < [u8] > > ;
    };
}

EndianArcSlice!();