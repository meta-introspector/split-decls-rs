// Generated macro for write_cargo_toml_header (function)
macro_rules! Depcrate_common_gen_rustwrite_cargo_toml_header {
() => {
// Module: crate::common::gen_rust
// Provides: {"write_cargo_toml_header"}
// Dependencies: {}
fn write_cargo_toml_header (w : & mut impl std :: io :: Write , name : & str) -> std :: io :: Result < () > { writeln ! (w , concatln ! ("[package]" , "name = \"{name}\"" , "version = \"{version}\"" , "authors = [{authors}]" , "license = \"{license}\"" , "edition = \"2018\"" ,) , name = name , version = env ! ("CARGO_PKG_VERSION") , authors = env ! ("CARGO_PKG_AUTHORS") . split (":") . format_with (", " , | author , fmt | fmt (& format_args ! ("\"{author}\""))) , license = env ! ("CARGO_PKG_LICENSE") ,) }
};
}
