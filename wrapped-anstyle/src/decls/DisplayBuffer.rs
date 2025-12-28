macro_rules! DisplayBuffer {
    () => {
        # [derive (Copy , Clone , Default , Debug)] struct DisplayBuffer { buffer : [u8 ; DISPLAY_BUFFER_CAPACITY] , len : usize , }
    };
}

DisplayBuffer!();