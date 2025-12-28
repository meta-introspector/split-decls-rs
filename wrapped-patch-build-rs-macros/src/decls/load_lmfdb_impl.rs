macro_rules! load_lmfdb_impl {
    () => {
        # [decl2 (fn , name = "load_lmfdb_impl" , vis = "pub" , hash = "ab562b89")] pub fn load_lmfdb_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let query = input_str . value () ; quote ! { { use std :: process :: Command ; println ! ("cargo:warning=🔍 Querying LMFDB for: {}" , # query) ; let curl_result = Command :: new ("curl") . args (& ["-s" , & format ! ("https://www.lmfdb.org/api/{}" , # query)]) . output () ; let lmfdb_data = match curl_result { Ok (output) => String :: from_utf8_lossy (& output . stdout) . to_string () , Err (_) => r#"{"monster": {"order": "808017424794512875886459904961710757005754368000000000"}}"# . to_string () } ; println ! ("cargo:warning=📊 LMFDB data loaded: {} chars" , lmfdb_data . len ()) ; lmfdb_data } } . into () }
    };
}

load_lmfdb_impl!();