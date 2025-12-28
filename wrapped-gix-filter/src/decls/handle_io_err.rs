macro_rules! deps {
    () => {
        Error!();
        Client!();
    };
}

macro_rules! handle_io_err {
    () => {
        deps!();
        pub (crate) fn handle_io_err (err : & std :: io :: Error , running : & mut HashMap < BString , process :: Client > , process : & BStr) { if matches ! (err . kind () , std :: io :: ErrorKind :: BrokenPipe | std :: io :: ErrorKind :: UnexpectedEof) { running . remove (process) . expect ("present or we wouldn't be here") ; } }
    };
}

handle_io_err!();