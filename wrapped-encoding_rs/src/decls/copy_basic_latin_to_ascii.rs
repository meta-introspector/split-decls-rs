macro_rules! copy_basic_latin_to_ascii {
    () => {
        # [doc = " Copies Basic Latin from source to destination narrowing it to ASCII up to"] # [doc = " the first non-Basic Latin code unit (or the end of the input if it is"] # [doc = " Basic Latin in its entirety)."] # [doc = ""] # [doc = " The length of the destination buffer must be at least the length of the"] # [doc = " source buffer."] # [doc = ""] # [doc = " Returns the number of bytes written."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the destination buffer is shorter than stated above."] pub fn copy_basic_latin_to_ascii (src : & [u16] , dst : & mut [u8]) -> usize { assert ! (dst . len () >= src . len () , "Destination must not be shorter than the source.") ; if let Some ((_ , consumed)) = unsafe { basic_latin_to_ascii (src . as_ptr () , dst . as_mut_ptr () , src . len ()) } { consumed } else { src . len () } }
    };
}

copy_basic_latin_to_ascii!();