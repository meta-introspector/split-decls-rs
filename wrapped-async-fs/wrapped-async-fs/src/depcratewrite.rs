// Generated macro for write (function)
macro_rules! Depcratewrite {
() => {
// Module: crate
// Provides: {"write"}
// Dependencies: {}
# [doc = " Writes a slice of bytes as the new contents of a file."] # [doc = ""] # [doc = " This function will create a file if it does not exist, and will entirely replace its contents"] # [doc = " if it does."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " An error will be returned in the following situations:"] # [doc = ""] # [doc = " * The file's parent directory does not exist."] # [doc = " * The current process lacks permissions to write to the file."] # [doc = " * Some other I/O error occurred."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " async_fs::write(\"a.txt\", b\"Hello world!\").await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub async fn write < P : AsRef < Path > , C : AsRef < [u8] > > (path : P , contents : C) -> io :: Result < () > { let path = path . as_ref () . to_owned () ; let contents = contents . as_ref () . to_owned () ; unblock (move | | std :: fs :: write (& path , contents)) . await }
};
}
