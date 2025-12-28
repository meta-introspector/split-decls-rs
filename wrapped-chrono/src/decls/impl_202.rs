macro_rules! deps {
    () => {
        Pad!();
        Colons!();
        FixedOffset!();
        OffsetFormat!();
        OffsetPrecision!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        # [cfg (any (feature = "alloc" , feature = "serde"))] impl OffsetFormat { # [doc = " Writes an offset from UTC with the format defined by `self`."] fn format (& self , w : & mut (impl Write + ? Sized) , off : FixedOffset) -> fmt :: Result { let off = off . local_minus_utc () ; if self . allow_zulu && off == 0 { w . write_char ('Z') ? ; return Ok (()) ; } let (sign , off) = if off < 0 { ('-' , - off) } else { ('+' , off) } ; let hours ; let mut mins = 0 ; let mut secs = 0 ; let precision = match self . precision { OffsetPrecision :: Hours => { hours = (off / 3600) as u8 ; OffsetPrecision :: Hours } OffsetPrecision :: Minutes | OffsetPrecision :: OptionalMinutes => { let minutes = (off + 30) / 60 ; mins = (minutes % 60) as u8 ; hours = (minutes / 60) as u8 ; if self . precision == OffsetPrecision :: OptionalMinutes && mins == 0 { OffsetPrecision :: Hours } else { OffsetPrecision :: Minutes } } OffsetPrecision :: Seconds | OffsetPrecision :: OptionalSeconds | OffsetPrecision :: OptionalMinutesAndSeconds => { let minutes = off / 60 ; secs = (off % 60) as u8 ; mins = (minutes % 60) as u8 ; hours = (minutes / 60) as u8 ; if self . precision != OffsetPrecision :: Seconds && secs == 0 { if self . precision == OffsetPrecision :: OptionalMinutesAndSeconds && mins == 0 { OffsetPrecision :: Hours } else { OffsetPrecision :: Minutes } } else { OffsetPrecision :: Seconds } } } ; let colons = self . colons == Colons :: Colon ; if hours < 10 { if self . padding == Pad :: Space { w . write_char (' ') ? ; } w . write_char (sign) ? ; if self . padding == Pad :: Zero { w . write_char ('0') ? ; } w . write_char ((b'0' + hours) as char) ? ; } else { w . write_char (sign) ? ; write_hundreds (w , hours) ? ; } if let OffsetPrecision :: Minutes | OffsetPrecision :: Seconds = precision { if colons { w . write_char (':') ? ; } write_hundreds (w , mins) ? ; } if let OffsetPrecision :: Seconds = precision { if colons { w . write_char (':') ? ; } write_hundreds (w , secs) ? ; } Ok (()) } }
    };
}

impl_202!();