macro_rules! find_rustc_source_impl {
    () => {
        # [decl (fn , name = "find_rustc_source_impl" , vis = "pub" , hash = "2ddd0c2a")] pub fn find_rustc_source_impl (input : TokenStream) -> TokenStream { let _input_str = parse_macro_input ! (input as LitStr) ; quote ! { { use std :: process :: Command ; use std :: path :: Path ; println ! ("cargo:warning=🔍 Finding real rustc source in Nix store") ; let nix_which = Command :: new ("which") . arg ("rustc") . output () ; let rustc_path = match nix_which { Ok (output) => String :: from_utf8_lossy (& output . stdout) . trim () . to_string () , Err (_) => "/usr/bin/rustc" . to_string () } ; let readlink = Command :: new ("readlink") . arg ("-f") . arg (& rustc_path) . output () ; let real_rustc_path = match readlink { Ok (output) => String :: from_utf8_lossy (& output . stdout) . trim () . to_string () , Err (_) => rustc_path . clone () } ; let nix_store_path = if real_rustc_path . starts_with ("/nix/store/") { let parts : Vec <& str > = real_rustc_path . split ('/') . collect () ; if parts . len () >= 4 { format ! ("/nix/store/{}" , parts [3]) } else { "/nix/store/unknown" . to_string () } } else { "/nix/store/not-found" . to_string () } ; let possible_sources = vec ! [format ! ("{}/src" , nix_store_path) , format ! ("{}/lib/rustlib/src/rust/compiler" , nix_store_path) , format ! ("{}/lib/rustlib/src/rust/src" , nix_store_path) , "/nix/store/*rust*/src" . to_string () ,] ; let mut found_sources = Vec :: new () ; for source_path in & possible_sources { if Path :: new (source_path) . exists () { found_sources . push (source_path . clone ()) ; } } let analysis = format ! (r#"
🔍 Real Rustc Source Analysis

📍 Rustc Binary Path: {}
📍 Real Path (after symlinks): {}
📍 Nix Store Path: {}

📂 Source Search Results:
{}

🔍 Next Steps:
1. Check if source exists at any of these paths
2. If not found, need to install rust source: nix-shell -p rustc.src
3. Or use: rustup component add rust-src

📊 Path Analysis:
- Binary found: {}
- Nix store detected: {}
- Source paths checked: {}
- Sources found: {}
                "# , rustc_path , real_rustc_path , nix_store_path , possible_sources . iter () . enumerate () . map (| (i , path) | format ! ("  {}. {}" , i + 1 , path)) . collect ::< Vec < _ >> () . join ("\n") , ! rustc_path . is_empty () , real_rustc_path . starts_with ("/nix/store/") , possible_sources . len () , found_sources . len ()) ; analysis } } . into () }
    };
}

find_rustc_source_impl!();