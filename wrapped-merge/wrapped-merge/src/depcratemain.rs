// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let time = std :: time :: Instant :: now () ; let mut output = None ; let mut input = vec ! [] ; let mut kind = ArgKind :: None ; for arg in std :: env :: args () . skip (1) { if arg . starts_with ('-') { kind = ArgKind :: None ; } match kind { ArgKind :: None => match arg . as_str () { "--in" => kind = ArgKind :: Input , "--out" => kind = ArgKind :: Output , _ => panic ! ("invalid option `{arg}`") , } , ArgKind :: Output => { if output . is_none () { output = Some (arg . to_string ()) ; } else { panic ! ("exactly one `--out` is required") ; } } ArgKind :: Input => input . push (arg . to_string ()) , } } let input = expand_input (input) ; let Some (output) = output else { panic ! ("exactly one `--out` is required") ; } ; let output = std :: path :: Path :: new (& output) ; let name = output . with_extension ("") ; let name = name . file_name () . expect ("`--out` file name is required") . to_string_lossy () ; let mut writer = writer :: File :: new (& name) ; let index = reader :: TypeIndex :: new (input) ; for ty in index . types () { write_type (& mut writer , & index , ty , None) ; } let bytes = writer . into_stream () ; std :: fs :: write (output , bytes) . unwrap () ; println ! ("Finished in {:.2}s" , time . elapsed () . as_secs_f32 ()) ; }
};
}
