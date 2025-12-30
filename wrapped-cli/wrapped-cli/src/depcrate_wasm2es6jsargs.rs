// Generated macro for Args (struct)
macro_rules! Depcrate_wasm2es6jsArgs {
() => {
// Module: crate::wasm2es6js
// Provides: {"Args"}
// Dependencies: {}
# [derive (Parser , Debug)] # [command (name = "wasm2es6js" , version , about , long_about = None , after_help = "Note that this is not intended to produce a production-ready output module but rather\n\
                  is intended purely as a temporary \"hack\" until it's standard in\n\
                  bundlers for working with wasm. Use this program with care!" ,)] struct Args { # [arg (long , short , value_name = "FILE" , help = "File to place output in")] output : Option < PathBuf > , # [arg (long , value_name = "DIR" , help = "Directory to place output in")] out_dir : Option < PathBuf > , # [arg (long , help = "Output a `*.d.ts` file next to the JS output")] typescript : bool , # [arg (long , help = "Inline the Wasm module using base64 encoding")] base64 : bool , # [arg (long , value_name = "PATH" , help = "Load module by passing the PATH argument to `fetch()`")] fetch : Option < String > , input : PathBuf , }
};
}
