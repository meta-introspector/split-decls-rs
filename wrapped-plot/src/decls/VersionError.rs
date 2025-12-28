macro_rules! VersionError {
    () => {
        # [doc = " Possible errors when parsing gnuplot's version string"] # [derive (Debug)] pub enum VersionError { # [doc = " The `gnuplot` command couldn't be executed"] Exec (io :: Error) , # [doc = " The `gnuplot` command returned an error message"] Error (String) , # [doc = " The `gnuplot` command returned invalid utf-8"] OutputError , # [doc = " The `gnuplot` command returned an unparsable string"] ParseError (String) , }
    };
}

VersionError!();