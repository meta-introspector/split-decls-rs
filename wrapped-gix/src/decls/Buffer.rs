macro_rules! deps {
    () => {
        Repository!();
        Clone!();
        Note!();
    };
}

macro_rules! Buffer {
    () => {
        deps!();
        # [doc = " A buffer that is returned to the free-list after usage."] # [derive (Clone)] pub struct Buffer < 'repo > { # [doc = " The buffer that would be returned to the freelist of `repo`."] # [doc = " Note that buffers without capacity (i.e. without allocation) aren't returned."] pub inner : Vec < u8 > , # [doc = " The repository from whose free-list the `inner` buffer was taken, and to which it will be returned."] pub repo : & 'repo crate :: Repository , }
    };
}

Buffer!();