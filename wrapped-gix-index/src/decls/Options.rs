macro_rules! deps {
    () => {
        Extensions!();
    };
}

macro_rules! Options {
    () => {
        deps!();
        # [doc = " The options for use when [writing an index][State::write_to()]."] # [doc = ""] # [doc = " Note that default options write either index V2 or V3 depending on the content of the entries."] # [derive (Debug , Default , Clone , Copy)] pub struct Options { # [doc = " Configures which extensions to write."] pub extensions : Extensions , # [doc = " Set the trailing hash of the produced index to all zeroes to save some time."] # [doc = ""] # [doc = " This value is typically controlled by `index.skipHash` and is respected when the index is written"] # [doc = " via [`File::write()`](crate::File::write()) and [`File::write_to()`](crate::File::write_to())."] # [doc = " Note that"] pub skip_hash : bool , }
    };
}

Options!()