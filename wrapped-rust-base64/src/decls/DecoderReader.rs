macro_rules! deps {
    () => {
        Engine!();
    };
}

macro_rules! DecoderReader {
    () => {
        deps!();
        # [doc = " A `Read` implementation that decodes base64 data read from an underlying reader."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io::Read;"] # [doc = " use std::io::Cursor;"] # [doc = " use base64::engine::general_purpose;"] # [doc = ""] # [doc = " // use a cursor as the simplest possible `Read` -- in real code this is probably a file, etc."] # [doc = " let mut wrapped_reader = Cursor::new(b\"YXNkZg==\");"] # [doc = " let mut decoder = base64::read::DecoderReader::new("] # [doc = "     &mut wrapped_reader,"] # [doc = "     &general_purpose::STANDARD);"] # [doc = ""] # [doc = " // handle errors as you normally would"] # [doc = " let mut result = Vec::new();"] # [doc = " decoder.read_to_end(&mut result).unwrap();"] # [doc = ""] # [doc = " assert_eq!(b\"asdf\", &result[..]);"] # [doc = ""] # [doc = " ```"] pub struct DecoderReader < 'e , E : Engine , R : io :: Read > { engine : & 'e E , # [doc = " Where b64 data is read from"] inner : R , # [doc = " Holds b64 data read from the delegate reader."] b64_buffer : [u8 ; BUF_SIZE] , # [doc = " The start of the pending buffered data in `b64_buffer`."] b64_offset : usize , # [doc = " The amount of buffered b64 data after `b64_offset` in `b64_len`."] b64_len : usize , # [doc = " Since the caller may provide us with a buffer of size 1 or 2 that's too small to copy a"] # [doc = " decoded chunk in to, we have to be able to hang on to a few decoded bytes."] # [doc = " Technically we only need to hold 2 bytes, but then we'd need a separate temporary buffer to"] # [doc = " decode 3 bytes into and then juggle copying one byte into the provided read buf and the rest"] # [doc = " into here, which seems like a lot of complexity for 1 extra byte of storage."] decoded_chunk_buffer : [u8 ; DECODED_CHUNK_SIZE] , # [doc = " Index of start of decoded data in `decoded_chunk_buffer`"] decoded_offset : usize , # [doc = " Length of decoded data after `decoded_offset` in `decoded_chunk_buffer`"] decoded_len : usize , # [doc = " Input length consumed so far."] # [doc = " Used to provide accurate offsets in errors"] input_consumed_len : usize , # [doc = " offset of previously seen padding, if any"] padding_offset : Option < usize > , }
    };
}

DecoderReader!()