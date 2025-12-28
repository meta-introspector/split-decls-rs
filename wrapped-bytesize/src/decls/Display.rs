macro_rules! deps {
    () => {
        ByteSize!();
        Format!();
    };
}

macro_rules! Display {
    () => {
        deps!();
        # [doc = " Formatting display wrapper for [`ByteSize`]."] # [doc = ""] # [doc = " Supports various styles, see methods. By default, the [`iec()`](Self::iec()) style is used."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use bytesize::ByteSize;"] # [doc = " assert_eq!("] # [doc = "     \"1.0 MiB\","] # [doc = "     ByteSize::mib(1).display().iec().to_string(),"] # [doc = " );"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     \"42.0k\","] # [doc = "     ByteSize::kb(42).display().si_short().to_string(),"] # [doc = " );"] # [doc = " ```"] # [derive (Debug , Clone)] pub struct Display { pub (crate) byte_size : ByteSize , pub (crate) format : Format , }
    };
}

Display!()