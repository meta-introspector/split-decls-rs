// Generated macro for Args (struct)
macro_rules! DepcrateArgs {
() => {
// Module: crate
// Provides: {"Args"}
// Dependencies: {}
# [derive (Parser , Debug)] # [clap (author , version , about , long_about = None)] struct Args { # [doc = " Path to the input corn file. If not set, reads from stdin instead."] input : Option < String > , # [doc = " The file format to output"] # [clap (long = "type" , short = 't' , value_enum)] output_type : Option < OutputType > , }
};
}
