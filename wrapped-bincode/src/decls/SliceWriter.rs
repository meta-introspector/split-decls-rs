macro_rules! deps {
    () => {
        Writer!();
    };
}

macro_rules! SliceWriter {
    () => {
        deps!();
        # [doc = " A helper struct that implements `Writer` for a `&[u8]` slice."] # [doc = ""] # [doc = " ```"] # [doc = " use bincode::enc::write::{Writer, SliceWriter};"] # [doc = ""] # [doc = " let destination = &mut [0u8; 100];"] # [doc = " let mut writer = SliceWriter::new(destination);"] # [doc = " writer.write(&[1, 2, 3, 4, 5]).unwrap();"] # [doc = ""] # [doc = " assert_eq!(writer.bytes_written(), 5);"] # [doc = " assert_eq!(destination[0..6], [1, 2, 3, 4, 5, 0]);"] # [doc = " ```"] pub struct SliceWriter < 'storage > { slice : & 'storage mut [u8] , original_length : usize , }
    };
}

SliceWriter!();