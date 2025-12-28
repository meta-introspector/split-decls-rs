macro_rules! deps {
    () => {
        AsyncSeekExt!();
        AsyncWriteExt!();
    };
}

macro_rules! Cursor {
    () => {
        deps!();
        # [doc = " Gives an in-memory buffer a cursor for reading and writing."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, Cursor, SeekFrom};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " let mut bytes = b\"hello\".to_vec();"] # [doc = " let mut cursor = Cursor::new(&mut bytes);"] # [doc = ""] # [doc = " // Overwrite 'h' with 'H'."] # [doc = " cursor.write_all(b\"H\").await?;"] # [doc = ""] # [doc = " // Move the cursor one byte forward."] # [doc = " cursor.seek(SeekFrom::Current(1)).await?;"] # [doc = ""] # [doc = " // Read a byte."] # [doc = " let mut byte = [0];"] # [doc = " cursor.read_exact(&mut byte).await?;"] # [doc = " assert_eq!(&byte, b\"l\");"] # [doc = ""] # [doc = " // Check the final buffer."] # [doc = " assert_eq!(bytes, b\"Hello\");"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] # [derive (Clone , Debug , Default)] pub struct Cursor < T > { inner : std :: io :: Cursor < T > , }
    };
}

Cursor!();