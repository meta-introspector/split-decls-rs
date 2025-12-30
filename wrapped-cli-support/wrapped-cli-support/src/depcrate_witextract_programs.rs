// Generated macro for extract_programs (function)
macro_rules! Depcrate_witextract_programs {
() => {
// Module: crate::wit
// Provides: {"extract_programs"}
// Dependencies: {}
# [doc = " Extract all of the `Program`s encoded in our custom section."] # [doc = ""] # [doc = " `program_storage` is used to squirrel away the raw bytes of the custom"] # [doc = "  section, so that they can be referenced by the `Program`s we return."] pub fn extract_programs < 'a > (module : & mut Module , program_storage : & 'a mut Vec < Vec < u8 > > ,) -> Result < Vec < decode :: Program < 'a > > , Error > { let my_version = wasm_bindgen_shared :: version () ; assert ! (program_storage . is_empty ()) ; while let Some (raw) = module . customs . remove_raw ("__wasm_bindgen_unstable") { log :: debug ! ("custom section '{}' looks like a Wasm bindgen section" , raw . name) ; program_storage . push (raw . data) ; } let mut ret = Vec :: new () ; for program in program_storage . iter () { let mut payload = & program [..] ; while let Some (data) = get_remaining (& mut payload) { if let Some (their_version) = verify_schema_matches (data) ? { bail ! ("

it looks like the Rust project used to create this Wasm file was linked against
version of wasm-bindgen that uses a different bindgen format than this binary:

  rust Wasm file schema version: {their_version}
     this binary schema version: {my_version}

Currently the bindgen format is unstable enough that these two schema versions
must exactly match. You can accomplish this by either updating this binary or
the wasm-bindgen dependency in the Rust project.

You should be able to update the wasm-bindgen dependency with:

    cargo update -p wasm-bindgen --precise {my_version}

don't forget to recompile your Wasm file! Alternatively, you can update the
binary with:

    cargo install -f wasm-bindgen-cli --version {their_version}

if this warning fails to go away though and you're not sure what to do feel free
to open an issue at https://github.com/wasm-bindgen/wasm-bindgen/issues!
") ; } let next = get_remaining (& mut payload) . unwrap () ; log :: debug ! ("found a program of length {}" , next . len ()) ; ret . push (< decode :: Program as decode :: Decode > :: decode_all (next)) ; } } Ok (ret) }
};
}
