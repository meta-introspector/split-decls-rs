macro_rules! trace_rustc_impl {
    () => {
        # [decl (fn , name = "trace_rustc_impl" , vis = "pub" , hash = "8d913e5d")] pub fn trace_rustc_impl (input : TokenStream) -> TokenStream { let _input_str = parse_macro_input ! (input as LitStr) ; quote ! { { use std :: process :: Command ; println ! ("cargo:warning=🔍 Tracing complete rustc system") ; let which_rustc = Command :: new ("which") . arg ("rustc") . output () . map (| o | String :: from_utf8_lossy (& o . stdout) . trim () . to_string ()) . unwrap_or_else (| _ | "rustc not found" . to_string ()) ; let real_path = Command :: new ("readlink") . args (& ["-f" , & which_rustc]) . output () . map (| o | String :: from_utf8_lossy (& o . stdout) . trim () . to_string ()) . unwrap_or_else (| _ | which_rustc . clone ()) ; let version_output = Command :: new ("rustc") . args (& ["--version" , "--verbose"]) . output () . map (| o | String :: from_utf8_lossy (& o . stdout) . to_string ()) . unwrap_or_else (| _ | "version unknown" . to_string ()) ; let commit_hash = version_output . lines () . find (| line | line . starts_with ("commit-hash:")) . and_then (| line | line . split (':') . nth (1)) . map (| s | s . trim () . to_string ()) . unwrap_or_else (|| "unknown" . to_string ()) ; let source_url = if ! commit_hash . is_empty () && commit_hash != "unknown" { format ! ("https://github.com/rust-lang/rust/archive/{}.tar.gz" , commit_hash) } else { "https://github.com/rust-lang/rust/archive/master.tar.gz" . to_string () } ; let possible_sources = vec ! [format ! ("{}/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust" , std :: env :: var ("HOME") . unwrap_or_else (| _ | "/tmp" . to_string ())) , "/nix/store/*rust*/src" . to_string () , "/usr/src/rust" . to_string () , "./rust-src" . to_string () ,] ; let mut found_source = None ; for source_path in & possible_sources { if std :: path :: Path :: new (source_path) . exists () { found_source = Some (source_path . clone ()) ; break ; } } let nix_info = if real_path . starts_with ("/nix/store/") { let store_path = real_path . split ('/') . take (4) . collect ::< Vec < _ >> () . join ("/") ; Command :: new ("nix") . args (& ["show-derivation" , & store_path]) . output () . map (| o | String :: from_utf8_lossy (& o . stdout) . to_string ()) . unwrap_or_else (| _ | "nix info unavailable" . to_string ()) } else { "not in nix store" . to_string () } ; let trace_report = format ! (r#"
🔍 COMPLETE RUSTC TRACE REPORT

1️⃣ RUSTC BINARY:
   Path: {}
   Real Path: {}
   In Nix Store: {}

2️⃣ VERSION INFO:
{}

3️⃣ SOURCE URL:
   GitHub: {}
   Commit: {}

4️⃣ SOURCE LOCATIONS CHECKED:
{}
   Found: {}

5️⃣ NIX STORE INFO:
{}

6️⃣ NEXT STEPS:
   - Download source: curl -L {} | tar xz
   - Or use rustup: rustup component add rust-src
   - Or use nix: nix-shell -p rustc.src
                "# , which_rustc , real_path , real_path . starts_with ("/nix/store/") , version_output , source_url , commit_hash , possible_sources . iter () . enumerate () . map (| (i , path) | format ! ("   {}. {}" , i + 1 , path)) . collect ::< Vec < _ >> () . join ("\n") , found_source . unwrap_or_else (|| "None found" . to_string ()) , if nix_info . len () > 100 { format ! ("{}..." , & nix_info [.. 100]) } else { nix_info }) ; trace_report } } . into () }
    };
}

trace_rustc_impl!();