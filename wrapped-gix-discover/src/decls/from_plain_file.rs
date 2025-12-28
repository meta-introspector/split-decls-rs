macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! from_plain_file {
    () => {
        deps!();
        # [doc = " Reads a plain path from a file that contains it as its only content, with trailing newlines trimmed."] pub fn from_plain_file (path : & std :: path :: Path) -> Option < std :: io :: Result < PathBuf > > { use bstr :: ByteSlice ; let mut buf = match read_regular_file_content_with_size_limit (path) { Ok (buf) => buf , Err (err) if err . kind () == std :: io :: ErrorKind :: NotFound => return None , Err (err) => return Some (Err (err)) , } ; let trimmed_len = buf . trim_end () . len () ; buf . truncate (trimmed_len) ; Some (Ok (gix_path :: from_bstring (buf))) }
    };
}

from_plain_file!();