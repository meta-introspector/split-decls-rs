// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl Args { fn parse () -> anyhow :: Result < Self > { let inner = Inner :: parse_from (wild :: args_os ()) ; let file_args = if ! inner . file . is_empty () { inner . file . clone () } else { vec ! ["-" . into ()] } ; if inner . raw && file_args . len () > 1 { bail ! ("Only one filename can be provided when using --raw") ; } let base_hasher = if inner . keyed { blake3 :: Hasher :: new_keyed (& read_key_from_stdin () ?) } else if let Some (ref context) = inner . derive_key { blake3 :: Hasher :: new_derive_key (context) } else { blake3 :: Hasher :: new () } ; Ok (Self { inner , file_args , base_hasher , }) } fn num_threads (& self) -> Option < usize > { self . inner . num_threads } fn check (& self) -> bool { self . inner . check } fn raw (& self) -> bool { self . inner . raw } fn tag (& self) -> bool { self . inner . tag } fn no_mmap (& self) -> bool { self . inner . no_mmap } fn no_names (& self) -> bool { self . inner . no_names } fn len (& self) -> u64 { self . inner . length } fn seek (& self) -> u64 { self . inner . seek } fn keyed (& self) -> bool { self . inner . keyed } fn quiet (& self) -> bool { self . inner . quiet } }
};
}
