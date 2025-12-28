macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! ReadData {
    () => {
        deps!();
        # [doc = " Lazy borrowed access to worktree or blob data, with streaming support for worktree files."] pub trait ReadData < 'a > { # [doc = " Returns the contents of this blob."] # [doc = ""] # [doc = " This potentially performs IO and other expensive operations"] # [doc = " and should only be called when necessary."] fn read_blob (self) -> Result < & 'a [u8] , Error > ; # [doc = " Stream a worktree file in such a manner that its content matches what would be put into git."] fn stream_worktree_file (self) -> Result < read_data :: Stream < 'a > , Error > ; }
    };
}

ReadData!();