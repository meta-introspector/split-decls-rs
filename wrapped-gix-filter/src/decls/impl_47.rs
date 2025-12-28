macro_rules! deps {
    () => {
        WriterThread!();
        Error!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl WriterThread { # [doc = " Spawn a thread that will write all data from `data` to `stdin`."] fn write_all_in_background (data : Vec < u8 > , mut stdin : std :: process :: ChildStdin) -> std :: io :: Result < Self > { let handle = std :: thread :: Builder :: new () . name ("gix-filter-stdin-writer" . into ()) . stack_size (128 * 1024) . spawn (move | | { use std :: io :: Write ; stdin . write_all (& data) ? ; drop (stdin) ; Ok (()) }) ? ; Ok (Self { handle : Some (handle) }) } # [doc = " Wait for the writer thread to complete and return any error that occurred."] fn join (& mut self) -> std :: io :: Result < () > { let Some (handle) = self . handle . take () else { return Ok (()) ; } ; handle . join () . map_err (| panic_info | { let msg = if let Some (s) = panic_info . downcast_ref :: < String > () { format ! ("Writer thread panicked: {s}") } else if let Some (s) = panic_info . downcast_ref :: < & str > () { format ! ("Writer thread panicked: {s}") } else { "Writer thread panicked while writing to filter stdin" . to_string () } ; std :: io :: Error :: other (msg) }) ? } }
    };
}

impl_47!();