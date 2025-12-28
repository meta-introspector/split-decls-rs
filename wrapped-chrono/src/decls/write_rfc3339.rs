macro_rules! deps {
    () => {
        Colons!();
        Pad!();
        FixedOffset!();
        OffsetPrecision!();
        NaiveDateTime!();
        SecondsFormat!();
        OffsetFormat!();
    };
}

macro_rules! write_rfc3339 {
    () => {
        deps!();
        # [doc = " Writes the date, time and offset to the string. same as `%Y-%m-%dT%H:%M:%S%.f%:z`"] # [inline] # [cfg (any (feature = "alloc" , feature = "serde"))] pub (crate) fn write_rfc3339 (w : & mut (impl Write + ? Sized) , dt : NaiveDateTime , off : FixedOffset , secform : SecondsFormat , use_z : bool ,) -> fmt :: Result { let year = dt . date () . year () ; if (0 ..= 9999) . contains (& year) { write_hundreds (w , (year / 100) as u8) ? ; write_hundreds (w , (year % 100) as u8) ? ; } else { write ! (w , "{year:+05}") ? ; } w . write_char ('-') ? ; write_hundreds (w , dt . date () . month () as u8) ? ; w . write_char ('-') ? ; write_hundreds (w , dt . date () . day () as u8) ? ; w . write_char ('T') ? ; let (hour , min , mut sec) = dt . time () . hms () ; let mut nano = dt . nanosecond () ; if nano >= 1_000_000_000 { sec += 1 ; nano -= 1_000_000_000 ; } write_hundreds (w , hour as u8) ? ; w . write_char (':') ? ; write_hundreds (w , min as u8) ? ; w . write_char (':') ? ; let sec = sec ; write_hundreds (w , sec as u8) ? ; match secform { SecondsFormat :: Secs => { } SecondsFormat :: Millis => write ! (w , ".{:03}" , nano / 1_000_000) ? , SecondsFormat :: Micros => write ! (w , ".{:06}" , nano / 1000) ? , SecondsFormat :: Nanos => write ! (w , ".{nano:09}") ? , SecondsFormat :: AutoSi => { if nano == 0 { } else if nano % 1_000_000 == 0 { write ! (w , ".{:03}" , nano / 1_000_000) ? } else if nano % 1_000 == 0 { write ! (w , ".{:06}" , nano / 1_000) ? } else { write ! (w , ".{nano:09}") ? } } SecondsFormat :: __NonExhaustive => unreachable ! () , } ; OffsetFormat { precision : OffsetPrecision :: Minutes , colons : Colons :: Colon , allow_zulu : use_z , padding : Pad :: Zero , } . format (w , off) }
    };
}

write_rfc3339!();