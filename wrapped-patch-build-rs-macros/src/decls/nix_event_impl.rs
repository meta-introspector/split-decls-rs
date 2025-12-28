macro_rules! nix_event_impl {
    () => {
        # [decl (fn , name = "nix_event_impl" , vis = "pub" , hash = "5eadfd4e")] pub fn nix_event_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let package = input_str . value () ; quote ! { { use std :: process :: Command ; let nix_info = Command :: new ("nix") . args (& ["search" , # package , "--json"]) . output () . map (| o | String :: from_utf8_lossy (& o . stdout) . to_string ()) . unwrap_or_else (| _ | format ! (r#"{{"packages":{{"{}":"found"}}}}"# , # package)) ; let memory_item = format ! ("MemoryItem::NixEvent {{ package: '{}', timestamp: {}, data: '{}' }}" , # package , std :: time :: SystemTime :: now () . duration_since (std :: time :: UNIX_EPOCH) . unwrap () . as_secs () , nix_info . chars () . take (100) . collect ::< String > ()) ; println ! ("cargo:warning=📦 Nix event: {}" , # package) ; memory_item } } . into () }
    };
}

nix_event_impl!()