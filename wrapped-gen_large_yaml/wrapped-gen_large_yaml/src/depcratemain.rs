// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> std :: io :: Result < () > { let mut generator = Generator :: new () ; let output_path = Path :: new (OUTPUT_DIR) ; if ! output_path . is_dir () { std :: fs :: create_dir (output_path) . unwrap () ; } println ! ("Generating big.yaml") ; let mut out = BufWriter :: new (File :: create (output_path . join ("big.yaml")) . unwrap ()) ; generator . gen_record_array (& mut out , 100_000 , 100_001) ? ; println ! ("Generating nested.yaml") ; let mut out = BufWriter :: new (File :: create (output_path . join ("nested.yaml")) . unwrap ()) ; nested :: create_deep_object (& mut out , 1_100_000) ? ; println ! ("Generating small_objects.yaml") ; let mut out = BufWriter :: new (File :: create (output_path . join ("small_objects.yaml")) . unwrap ()) ; generator . gen_authors_array (& mut out , 4_000_000 , 4_000_001) ? ; println ! ("Generating strings_array.yaml") ; let mut out = BufWriter :: new (File :: create (output_path . join ("strings_array.yaml")) . unwrap ()) ; generator . gen_strings_array (& mut out , 1_300_000 , 1_300_001 , 10 , 40) ? ; Ok (()) }
};
}
