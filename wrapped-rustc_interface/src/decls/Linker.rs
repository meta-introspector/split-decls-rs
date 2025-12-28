macro_rules! Linker {
    () => {
        pub struct Linker { dep_graph : DepGraph , output_filenames : Arc < OutputFilenames > , crate_hash : Option < Svh > , metadata : EncodedMetadata , ongoing_codegen : Box < dyn Any > , }
    };
}

Linker!();