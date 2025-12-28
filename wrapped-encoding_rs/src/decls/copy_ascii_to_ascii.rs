macro_rules! copy_ascii_to_ascii {
    () => {
        # [doc = " Copies ASCII from source to destination up to the first non-ASCII byte"] # [doc = " (or the end of the input if it is ASCII in its entirety)."] # [doc = ""] # [doc = " The length of the destination buffer must be at least the length of the"] # [doc = " source buffer."] # [doc = ""] # [doc = " Returns the number of bytes written."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the destination buffer is shorter than stated above."] pub fn copy_ascii_to_ascii (src : & [u8] , dst : & mut [u8]) -> usize { assert ! (dst . len () >= src . len () , "Destination must not be shorter than the source.") ; if let Some ((_ , consumed)) = unsafe { ascii_to_ascii (src . as_ptr () , dst . as_mut_ptr () , src . len ()) } { consumed } else { src . len () } }
    };
}

copy_ascii_to_ascii!()